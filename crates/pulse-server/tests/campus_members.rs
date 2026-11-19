mod common;

use axum::http::{Method, StatusCode};
use common::{
    api, campus_for, expect_close, fresh_state, grant_role, joined_socket, signed_up,
    spawn_server_with,
};
use pulse_protocol::{
    AccountId, AddMemberRequest, AuthSession, CampusId, CampusMember, MemberList, Role,
    UpdateMemberRequest, api_path, close_code,
};
use pulse_server::AppState;
use rstest::rstest;
use serde_json::{Value, json};

struct Cast {
    state: AppState,
    campus_id: CampusId,
    owner: AuthSession,
    admin: AuthSession,
    member: AuthSession,
    outsider: AuthSession,
}

#[derive(Clone, Copy, Debug)]
enum Who {
    Owner,
    Admin,
    Member,
    Outsider,
}

impl Cast {
    async fn open() -> Self {
        let state = fresh_state().await;
        let owner = signed_up(&state, "owner@example.org").await;
        let admin = signed_up(&state, "admin@example.org").await;
        let member = signed_up(&state, "member@example.org").await;
        let outsider = signed_up(&state, "outsider@example.org").await;
        let campus_id = campus_for(&state, &owner.token, "north-quad")
            .await
            .campus
            .campus_id;
        grant_role(&state, campus_id, admin.account.account_id, Role::Admin).await;
        grant_role(&state, campus_id, member.account.account_id, Role::Member).await;
        Self {
            state,
            campus_id,
            owner,
            admin,
            member,
            outsider,
        }
    }

    fn session(&self, who: Who) -> &AuthSession {
        match who {
            Who::Owner => &self.owner,
            Who::Admin => &self.admin,
            Who::Member => &self.member,
            Who::Outsider => &self.outsider,
        }
    }

    fn id(&self, who: Who) -> AccountId {
        self.session(who).account.account_id
    }

    async fn call(
        &self,
        actor: Who,
        method: Method,
        path: &str,
        body: Option<Value>,
    ) -> (StatusCode, Value) {
        api(
            &self.state,
            method,
            path,
            Some(&self.session(actor).token),
            body.as_ref(),
        )
        .await
    }

    async fn roles(&self) -> Vec<(String, Role)> {
        let (status, body) = self
            .call(
                Who::Owner,
                Method::GET,
                &api_path::campus_members(self.campus_id),
                None,
            )
            .await;
        assert_eq!(status, StatusCode::OK, "{body}");
        let list: MemberList = serde_json::from_value(body).expect("member list");
        list.members
            .into_iter()
            .map(|member| (member.email, member.role))
            .collect()
    }
}

#[derive(Clone, Copy, Debug)]
enum Action {
    List,
    Add(Role),
    Update(Who, Role),
    Remove(Who),
}

#[rstest]
#[case::owner_lists(Who::Owner, Action::List, StatusCode::OK)]
#[case::member_lists(Who::Member, Action::List, StatusCode::OK)]
#[case::outsider_lists(Who::Outsider, Action::List, StatusCode::NOT_FOUND)]
#[case::owner_adds_admin(Who::Owner, Action::Add(Role::Admin), StatusCode::CREATED)]
#[case::admin_adds_member(Who::Admin, Action::Add(Role::Member), StatusCode::CREATED)]
#[case::admin_adds_admin(Who::Admin, Action::Add(Role::Admin), StatusCode::FORBIDDEN)]
#[case::member_adds_member(Who::Member, Action::Add(Role::Member), StatusCode::FORBIDDEN)]
#[case::owner_adds_owner(Who::Owner, Action::Add(Role::Owner), StatusCode::BAD_REQUEST)]
#[case::owner_demotes_admin(Who::Owner, Action::Update(Who::Admin, Role::Member), StatusCode::OK)]
#[case::admin_demotes_member(Who::Admin, Action::Update(Who::Member, Role::Guest), StatusCode::OK)]
#[case::admin_promotes_member(
    Who::Admin,
    Action::Update(Who::Member, Role::Admin),
    StatusCode::FORBIDDEN
)]
#[case::admin_demotes_owner(
    Who::Admin,
    Action::Update(Who::Owner, Role::Member),
    StatusCode::FORBIDDEN
)]
#[case::owner_demotes_self(
    Who::Owner,
    Action::Update(Who::Owner, Role::Member),
    StatusCode::FORBIDDEN
)]
#[case::member_demotes_member(
    Who::Member,
    Action::Update(Who::Member, Role::Guest),
    StatusCode::FORBIDDEN
)]
#[case::owner_updates_outsider(
    Who::Owner,
    Action::Update(Who::Outsider, Role::Guest),
    StatusCode::NOT_FOUND
)]
#[case::owner_removes_admin(Who::Owner, Action::Remove(Who::Admin), StatusCode::NO_CONTENT)]
#[case::admin_removes_member(Who::Admin, Action::Remove(Who::Member), StatusCode::NO_CONTENT)]
#[case::admin_removes_admin(Who::Admin, Action::Remove(Who::Admin), StatusCode::FORBIDDEN)]
#[case::admin_removes_owner(Who::Admin, Action::Remove(Who::Owner), StatusCode::FORBIDDEN)]
#[case::member_removes_member(Who::Member, Action::Remove(Who::Member), StatusCode::FORBIDDEN)]
#[tokio::test]
async fn member_management_matrix(
    #[case] actor: Who,
    #[case] action: Action,
    #[case] status: StatusCode,
) {
    let cast = Cast::open().await;
    let before = cast.roles().await;
    let members_path = api_path::campus_members(cast.campus_id);
    let (actual, body) = match action {
        Action::List => cast.call(actor, Method::GET, &members_path, None).await,
        Action::Add(role) => {
            let request = json!(AddMemberRequest {
                email: "Outsider@Example.org ".to_owned(),
                role
            });
            cast.call(actor, Method::POST, &members_path, Some(request))
                .await
        }
        Action::Update(target, role) => {
            let path = api_path::campus_member(cast.campus_id, cast.id(target));
            let request = json!(UpdateMemberRequest { role });
            cast.call(actor, Method::PATCH, &path, Some(request)).await
        }
        Action::Remove(target) => {
            let path = api_path::campus_member(cast.campus_id, cast.id(target));
            cast.call(actor, Method::DELETE, &path, None).await
        }
    };
    assert_eq!(actual, status, "{body}");
    let after = cast.roles().await;
    let mut expected = before.clone();
    if status.is_success() {
        match action {
            Action::List => {
                let list: MemberList = serde_json::from_value(body).expect("member list");
                assert_eq!(list.members.len(), 3);
            }
            Action::Add(role) => {
                let member: CampusMember = serde_json::from_value(body).expect("member");
                assert_eq!(member.email, "outsider@example.org");
                expected.push(("outsider@example.org".to_owned(), role));
            }
            Action::Update(target, role) => {
                let email = cast.session(target).account.email.clone();
                for entry in &mut expected {
                    if entry.0 == email {
                        entry.1 = role;
                    }
                }
            }
            Action::Remove(target) => {
                let email = cast.session(target).account.email.clone();
                expected.retain(|entry| entry.0 != email);
            }
        }
    }
    expected.sort_by(|left, right| left.0.cmp(&right.0));
    let mut sorted = after;
    sorted.sort_by(|left, right| left.0.cmp(&right.0));
    assert_eq!(sorted, expected);
}

#[rstest]
#[case::unknown_email("nobody@example.org", StatusCode::NOT_FOUND, json!({"code": "not_found", "field": "email"}))]
#[case::existing_member_keeps_role("member@example.org", StatusCode::OK, json!("member"))]
#[tokio::test]
async fn adding_by_email_edge_cases(
    #[case] email: &str,
    #[case] status: StatusCode,
    #[case] expected: Value,
) {
    let cast = Cast::open().await;
    let request = json!(AddMemberRequest {
        email: email.to_owned(),
        role: Role::Guest
    });
    let (actual, body) = cast
        .call(
            Who::Owner,
            Method::POST,
            &api_path::campus_members(cast.campus_id),
            Some(request),
        )
        .await;
    assert_eq!(actual, status, "{body}");
    let observed = if status.is_success() {
        body["role"].clone()
    } else {
        body
    };
    assert_eq!(observed, expected);
}

#[tokio::test]
async fn removed_member_socket_is_closed() {
    let cast = Cast::open().await;
    let addr = spawn_server_with(cast.state.clone()).await;
    let (mut removed, _) = joined_socket(addr, cast.campus_id, &cast.member.token).await;
    let path = api_path::campus_member(cast.campus_id, cast.id(Who::Member));
    let (status, body) = cast.call(Who::Owner, Method::DELETE, &path, None).await;
    assert_eq!(status, StatusCode::NO_CONTENT, "{body}");
    assert_eq!(
        expect_close(&mut removed).await,
        close_code::POLICY_VIOLATION
    );
}
