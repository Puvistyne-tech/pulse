use std::collections::BTreeMap;
use std::sync::LazyLock;

use pulse_protocol::{
    AccountId, CampusId, LOUNGE_PARTICIPANTS_MAX, LOUNGE_PRESENCE_TTL_MS, LOUNGE_TICKET_TTL_MS,
    Lounge, LoungeId, LoungeJoined, LoungeParticipant, LoungeRejection, LoungeScope, LoungeTicket,
    Permission, Role, RoomId, SessionId, role_permits,
};
use redis::Script;
use uuid::Uuid;

use crate::livekit::{LiveKitClaims, VideoGrant, livekit_room_name, sign_livekit_token};
use crate::state::AppState;
use crate::time::now_ms;

const PRUNE_AND_TIME: &str = "local t = redis.call('TIME') \
    local now = tonumber(t[1]) * 1000 + math.floor(tonumber(t[2]) / 1000) \
    redis.call('ZREMRANGEBYSCORE', KEYS[1], '-inf', now) ";

const RETURN_STATE: &str = "local out = {tostring(redis.call('INCR', KEYS[2]))} \
    local members = redis.call('ZRANGE', KEYS[1], 0, -1, 'WITHSCORES') \
    for i = 1, #members do out[#out + 1] = members[i] end \
    return out";

static PRESENT: LazyLock<Script> = LazyLock::new(|| {
    Script::new(&format!(
        "{PRUNE_AND_TIME} \
         local present = redis.call('ZSCORE', KEYS[1], ARGV[4] .. '0') \
         or redis.call('ZSCORE', KEYS[1], ARGV[4] .. '1') \
         if not present and redis.call('ZCARD', KEYS[1]) >= tonumber(ARGV[3]) then \
         return {{'-1'}} end \
         redis.call('ZREM', KEYS[1], ARGV[4] .. '0', ARGV[4] .. '1') \
         redis.call('ZADD', KEYS[1], now + tonumber(ARGV[2]), ARGV[1]) \
         redis.call('PEXPIRE', KEYS[1], tonumber(ARGV[2]) * 2) \
         {RETURN_STATE}"
    ))
});

static ABSENT: LazyLock<Script> = LazyLock::new(|| {
    Script::new(&format!(
        "{PRUNE_AND_TIME} \
         redis.call('ZREM', KEYS[1], ARGV[1] .. '0', ARGV[1] .. '1') \
         redis.call('PEXPIRE', KEYS[1], tonumber(ARGV[2]) * 2) \
         {RETURN_STATE}"
    ))
});

static SNAPSHOT: LazyLock<Script> = LazyLock::new(|| {
    Script::new(
        "local t = redis.call('TIME') \
         local now = tonumber(t[1]) * 1000 + math.floor(tonumber(t[2]) / 1000) \
         local out = {} \
         for i = 1, #KEYS, 2 do \
         local members = redis.call('ZRANGEBYSCORE', KEYS[i], '(' .. now, '+inf', 'WITHSCORES') \
         out[#out + 1] = tostring(tonumber(redis.call('GET', KEYS[i + 1])) or 0) \
         out[#out + 1] = tostring(#members / 2) \
         for j = 1, #members do out[#out + 1] = members[j] end \
         end \
         return out",
    )
});

#[derive(Clone)]
pub struct LoungeConfig {
    pub livekit_url: String,
    pub api_key: String,
    pub api_secret: String,
    pub presence_ttl_ms: u64,
    pub ticket_ttl_ms: u64,
}

impl std::fmt::Debug for LoungeConfig {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("LoungeConfig")
            .field("livekit_url", &self.livekit_url)
            .field("api_key", &self.api_key)
            .field("api_secret", &"redacted")
            .field("presence_ttl_ms", &self.presence_ttl_ms)
            .field("ticket_ttl_ms", &self.ticket_ttl_ms)
            .finish()
    }
}

impl Default for LoungeConfig {
    fn default() -> Self {
        Self {
            livekit_url: "ws://127.0.0.1:7880".to_owned(),
            api_key: "devkey".to_owned(),
            api_secret: "secret".to_owned(),
            presence_ttl_ms: LOUNGE_PRESENCE_TTL_MS,
            ticket_ttl_ms: LOUNGE_TICKET_TTL_MS,
        }
    }
}

pub fn presence_key(campus_id: CampusId, lounge_id: LoungeId) -> String {
    format!("pulse:campus:{campus_id}:lounge:{lounge_id}:presence")
}

pub fn revision_key(campus_id: CampusId, lounge_id: LoungeId) -> String {
    format!("pulse:campus:{campus_id}:lounge:{lounge_id}:revision")
}

#[derive(Debug)]
pub enum LoungeFailure {
    Rejected(LoungeRejection),
    Database(sqlx::Error),
}

impl From<sqlx::Error> for LoungeFailure {
    fn from(error: sqlx::Error) -> Self {
        LoungeFailure::Database(error)
    }
}

impl From<redis::RedisError> for LoungeFailure {
    fn from(error: redis::RedisError) -> Self {
        tracing::warn!(%error, "lounge presence unavailable");
        LoungeFailure::Rejected(LoungeRejection::Unavailable)
    }
}

#[derive(Debug, Default)]
struct Presence {
    revision: u64,
    members: Vec<(AccountId, bool, u64)>,
}

fn parse_member(member: &str, score: &str) -> Option<(AccountId, bool, u64)> {
    let mut parts = member.split(':');
    let account = parts.next()?.parse::<Uuid>().ok()?;
    parts.next()?;
    let can_speak = parts.next()? == "1";
    let until = score.parse::<f64>().ok()?;
    Some((AccountId(account), can_speak, until as u64))
}

fn parse_members(tokens: &[String]) -> Vec<(AccountId, bool, u64)> {
    tokens
        .as_chunks::<2>()
        .0
        .iter()
        .filter_map(|[member, score]| parse_member(member, score))
        .collect()
}

fn parse_presence(tokens: &[String]) -> Option<Presence> {
    let revision = tokens.first()?.parse::<i64>().ok()?;
    let revision = u64::try_from(revision).ok()?;
    Some(Presence {
        revision,
        members: parse_members(&tokens[1..]),
    })
}

async fn lounge_scope(
    state: &AppState,
    campus_id: CampusId,
    lounge_id: LoungeId,
) -> Result<Option<Option<RoomId>>, sqlx::Error> {
    let row = sqlx::query_scalar::<_, Option<Uuid>>(
        "SELECT room_id FROM lounges WHERE campus_id = $1 AND lounge_id = $2",
    )
    .bind(campus_id.as_uuid())
    .bind(lounge_id.as_uuid())
    .fetch_optional(&state.db)
    .await?;
    Ok(row.map(|room_id| room_id.map(RoomId)))
}

async fn display_names(
    state: &AppState,
    campus_id: CampusId,
    account_ids: &[AccountId],
) -> Result<BTreeMap<AccountId, String>, sqlx::Error> {
    let ids: Vec<Uuid> = account_ids.iter().map(|id| id.as_uuid()).collect();
    let rows = sqlx::query_as::<_, (Uuid, String)>(
        "SELECT a.account_id, a.display_name FROM accounts a JOIN campus_members m \
         ON m.account_id = a.account_id \
         WHERE m.campus_id = $1 AND a.account_id = ANY($2)",
    )
    .bind(campus_id.as_uuid())
    .bind(&ids)
    .fetch_all(&state.db)
    .await?;
    Ok(rows
        .into_iter()
        .map(|(account_id, name)| (AccountId(account_id), name))
        .collect())
}

fn build_lounge(
    campus_id: CampusId,
    lounge_id: LoungeId,
    room_id: Option<RoomId>,
    presence: &Presence,
    names: &BTreeMap<AccountId, String>,
) -> Lounge {
    let mut merged: BTreeMap<AccountId, (bool, u64)> = BTreeMap::new();
    for (account_id, can_speak, until) in &presence.members {
        let entry = merged.entry(*account_id).or_insert((false, 0));
        entry.0 |= *can_speak;
        entry.1 = entry.1.max(*until);
    }
    let participants = merged
        .into_iter()
        .filter_map(|(account_id, (can_speak, until))| {
            names.get(&account_id).map(|name| LoungeParticipant {
                account_id,
                display_name: name.clone(),
                can_speak,
                present_until_ms: until,
            })
        })
        .collect();
    Lounge {
        lounge_id,
        campus_id,
        scope: if room_id.is_some() {
            LoungeScope::Room
        } else {
            LoungeScope::Campus
        },
        room_id,
        participants,
        revision: presence.revision,
    }
}

fn member_of(account_id: AccountId, session_id: SessionId, can_speak: bool) -> String {
    format!("{account_id}:{session_id}:{}", u8::from(can_speak))
}

async fn named_lounge(
    state: &AppState,
    campus_id: CampusId,
    lounge_id: LoungeId,
    room_id: Option<RoomId>,
    presence: &Presence,
) -> Result<Lounge, LoungeFailure> {
    let accounts: Vec<AccountId> = presence.members.iter().map(|member| member.0).collect();
    let names = display_names(state, campus_id, &accounts).await?;
    Ok(build_lounge(
        campus_id, lounge_id, room_id, presence, &names,
    ))
}

pub async fn lounges_of(
    state: &AppState,
    campus_id: CampusId,
) -> Result<Vec<Lounge>, LoungeFailure> {
    let rows = sqlx::query_as::<_, (Uuid, Option<Uuid>)>(
        "SELECT lounge_id, room_id FROM lounges WHERE campus_id = $1 \
         ORDER BY (room_id IS NOT NULL), lounge_id",
    )
    .bind(campus_id.as_uuid())
    .fetch_all(&state.db)
    .await?;
    if rows.is_empty() {
        return Ok(Vec::new());
    }
    let mut connection = state.valkey.get_multiplexed_async_connection().await?;
    let mut invocation = SNAPSHOT.prepare_invoke();
    for (lounge_id, _) in &rows {
        let lounge_id = LoungeId(*lounge_id);
        invocation
            .key(presence_key(campus_id, lounge_id))
            .key(revision_key(campus_id, lounge_id));
    }
    let tokens: Vec<String> = invocation.invoke_async(&mut connection).await?;
    let mut cursor = tokens.into_iter();
    let mut presences = Vec::with_capacity(rows.len());
    for _ in &rows {
        let revision = cursor
            .next()
            .and_then(|token| token.parse::<u64>().ok())
            .unwrap_or(0);
        let count = cursor
            .next()
            .and_then(|token| token.parse::<f64>().ok())
            .map_or(0, |count| count as usize);
        let flat: Vec<String> = cursor.by_ref().take(count * 2).collect();
        presences.push(Presence {
            revision,
            members: parse_members(&flat),
        });
    }
    let everyone: Vec<AccountId> = presences
        .iter()
        .flat_map(|presence| presence.members.iter().map(|member| member.0))
        .collect();
    let names = display_names(state, campus_id, &everyone).await?;
    Ok(rows
        .into_iter()
        .zip(presences)
        .map(|((lounge_id, room_id), presence)| {
            build_lounge(
                campus_id,
                LoungeId(lounge_id),
                room_id.map(RoomId),
                &presence,
                &names,
            )
        })
        .collect())
}

async fn mark_present(
    state: &AppState,
    campus_id: CampusId,
    account_id: AccountId,
    session_id: SessionId,
    role: Role,
    lounge_id: LoungeId,
) -> Result<Lounge, LoungeFailure> {
    if !role_permits(role, Permission::JoinLounge) {
        return Err(LoungeFailure::Rejected(LoungeRejection::Forbidden));
    }
    let Some(room_id) = lounge_scope(state, campus_id, lounge_id).await? else {
        return Err(LoungeFailure::Rejected(LoungeRejection::UnknownLounge));
    };
    let member = member_of(
        account_id,
        session_id,
        role_permits(role, Permission::SpeakInLounge),
    );
    let mut connection = state.valkey.get_multiplexed_async_connection().await?;
    let tokens: Vec<String> = PRESENT
        .key(presence_key(campus_id, lounge_id))
        .key(revision_key(campus_id, lounge_id))
        .arg(member)
        .arg(state.lounge.presence_ttl_ms)
        .arg(LOUNGE_PARTICIPANTS_MAX)
        .arg(format!("{account_id}:{session_id}:"))
        .invoke_async(&mut connection)
        .await?;
    let Some(presence) = parse_presence(&tokens) else {
        return Err(LoungeFailure::Rejected(LoungeRejection::Full));
    };
    named_lounge(state, campus_id, lounge_id, room_id, &presence).await
}

fn ticket_for(
    state: &AppState,
    campus_id: CampusId,
    account_id: AccountId,
    role: Role,
    lounge: &Lounge,
) -> LoungeTicket {
    let name = lounge
        .participants
        .iter()
        .find(|participant| participant.account_id == account_id)
        .map(|participant| participant.display_name.clone())
        .unwrap_or_default();
    let not_before = now_ms() / 1000;
    let expires = not_before.saturating_add(state.lounge.ticket_ttl_ms / 1000);
    let room_name = livekit_room_name(campus_id, lounge.lounge_id);
    let claims = LiveKitClaims {
        iss: state.lounge.api_key.clone(),
        sub: account_id.to_string(),
        name,
        nbf: not_before,
        exp: expires,
        video: VideoGrant {
            room: room_name.clone(),
            room_join: true,
            can_publish: role_permits(role, Permission::SpeakInLounge),
            can_subscribe: true,
            can_publish_data: false,
        },
    };
    LoungeTicket {
        lounge_id: lounge.lounge_id,
        livekit_url: state.lounge.livekit_url.clone(),
        room_name,
        token: sign_livekit_token(&claims, &state.lounge.api_secret),
        expires_at_ms: expires.saturating_mul(1000),
    }
}

pub async fn join_lounge(
    state: &AppState,
    campus_id: CampusId,
    account_id: AccountId,
    session_id: SessionId,
    role: Role,
    lounge_id: LoungeId,
) -> Result<LoungeJoined, LoungeFailure> {
    let lounge = mark_present(state, campus_id, account_id, session_id, role, lounge_id).await?;
    let ticket = ticket_for(state, campus_id, account_id, role, &lounge);
    Ok(LoungeJoined { lounge, ticket })
}

pub async fn heartbeat_lounge(
    state: &AppState,
    campus_id: CampusId,
    account_id: AccountId,
    session_id: SessionId,
    role: Role,
    lounge_id: LoungeId,
) -> Result<Lounge, LoungeFailure> {
    mark_present(state, campus_id, account_id, session_id, role, lounge_id).await
}

pub async fn leave_lounge(
    state: &AppState,
    campus_id: CampusId,
    account_id: AccountId,
    session_id: SessionId,
    lounge_id: LoungeId,
) -> Result<Lounge, LoungeFailure> {
    let Some(room_id) = lounge_scope(state, campus_id, lounge_id).await? else {
        return Err(LoungeFailure::Rejected(LoungeRejection::UnknownLounge));
    };
    let mut connection = state.valkey.get_multiplexed_async_connection().await?;
    let tokens: Vec<String> = ABSENT
        .key(presence_key(campus_id, lounge_id))
        .key(revision_key(campus_id, lounge_id))
        .arg(format!("{account_id}:{session_id}:"))
        .arg(state.lounge.presence_ttl_ms)
        .invoke_async(&mut connection)
        .await?;
    let Some(presence) = parse_presence(&tokens) else {
        return Err(LoungeFailure::Rejected(LoungeRejection::Unavailable));
    };
    named_lounge(state, campus_id, lounge_id, room_id, &presence).await
}
