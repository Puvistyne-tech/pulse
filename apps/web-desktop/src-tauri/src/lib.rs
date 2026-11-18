use pulse_core::native::NativeClient;
use percent_encoding::percent_decode_str;
use pulse_protocol::{
    CampusId, ClientNonce, ClientSnapshot, CreateCampusRequest, DraftId, InboxStep, MessageId,
    ReactionAction, ReminderPreset, RoomId, ShelfAction, SignInRequest, SignUpRequest,
    SubscriptionState, VaultTab,
};
use tauri::ipc::InvokeBody;
use tauri::{Manager, State};

const ROOM_ID_HEADER: &str = "pulse-room-id";
const FILE_NAME_HEADER: &str = "pulse-file-name";

type CommandResult = Result<ClientSnapshot, String>;

#[tauri::command]
async fn client_snapshot(client: State<'_, NativeClient>) -> CommandResult {
    Ok(client.snapshot().await)
}

#[tauri::command]
async fn sign_up(client: State<'_, NativeClient>, request: SignUpRequest) -> CommandResult {
    Ok(client.sign_up(request).await)
}

#[tauri::command]
async fn sign_in(client: State<'_, NativeClient>, request: SignInRequest) -> CommandResult {
    Ok(client.sign_in(request).await)
}

#[tauri::command]
async fn sign_out(client: State<'_, NativeClient>) -> CommandResult {
    Ok(client.sign_out().await)
}

#[tauri::command]
async fn refresh_campuses(client: State<'_, NativeClient>) -> CommandResult {
    Ok(client.refresh_campuses().await)
}

#[tauri::command]
async fn create_campus(
    client: State<'_, NativeClient>,
    request: CreateCampusRequest,
) -> CommandResult {
    Ok(client.create_campus(request).await)
}

#[tauri::command]
async fn select_campus(client: State<'_, NativeClient>, campus_id: CampusId) -> CommandResult {
    Ok(client.select_campus(campus_id).await)
}

#[tauri::command]
async fn select_room(client: State<'_, NativeClient>, room_id: RoomId) -> CommandResult {
    Ok(client.select_room(room_id).await)
}

#[tauri::command]
async fn send_message(
    client: State<'_, NativeClient>,
    room_id: RoomId,
    body: String,
) -> CommandResult {
    Ok(client.send_message(room_id, body).await)
}

#[tauri::command]
async fn discard_message(
    client: State<'_, NativeClient>,
    client_nonce: ClientNonce,
) -> CommandResult {
    Ok(client.discard_message(client_nonce).await)
}

#[tauri::command]
async fn send_reply(
    client: State<'_, NativeClient>,
    room_id: RoomId,
    root_message_id: MessageId,
    body: String,
) -> CommandResult {
    Ok(client.send_reply(room_id, root_message_id, body).await)
}

#[tauri::command]
async fn react(
    client: State<'_, NativeClient>,
    room_id: RoomId,
    message_id: MessageId,
    emoji: String,
    action: ReactionAction,
) -> CommandResult {
    Ok(client.react(room_id, message_id, emoji, action).await)
}

#[tauri::command]
async fn set_topic_title(
    client: State<'_, NativeClient>,
    room_id: RoomId,
    root_message_id: MessageId,
    topic_title: String,
) -> CommandResult {
    Ok(client
        .set_topic_title(room_id, root_message_id, topic_title)
        .await)
}

#[tauri::command]
async fn set_thread_subscription(
    client: State<'_, NativeClient>,
    room_id: RoomId,
    root_message_id: MessageId,
    state: SubscriptionState,
) -> CommandResult {
    Ok(client
        .set_thread_subscription(room_id, root_message_id, state)
        .await)
}

#[tauri::command]
async fn open_thread(
    client: State<'_, NativeClient>,
    room_id: RoomId,
    root_message_id: MessageId,
) -> CommandResult {
    Ok(client.open_thread(room_id, root_message_id).await)
}

#[tauri::command]
async fn close_thread(client: State<'_, NativeClient>) -> CommandResult {
    Ok(client.close_thread().await)
}

#[tauri::command]
async fn toggle_inbox(client: State<'_, NativeClient>) -> CommandResult {
    Ok(client.toggle_inbox().await)
}

#[tauri::command]
async fn move_inbox_selection(client: State<'_, NativeClient>, step: InboxStep) -> CommandResult {
    Ok(client.move_inbox_selection(step).await)
}

#[tauri::command]
async fn archive_inbox_selection(client: State<'_, NativeClient>) -> CommandResult {
    Ok(client.archive_inbox_selection().await)
}

#[tauri::command]
async fn start_inbox_reply(client: State<'_, NativeClient>) -> CommandResult {
    Ok(client.start_inbox_reply().await)
}

#[tauri::command]
async fn cancel_inbox_reply(client: State<'_, NativeClient>) -> CommandResult {
    Ok(client.cancel_inbox_reply().await)
}

#[tauri::command]
async fn toggle_search(client: State<'_, NativeClient>) -> CommandResult {
    Ok(client.toggle_search().await)
}

#[tauri::command]
async fn search(client: State<'_, NativeClient>, query: String) -> CommandResult {
    Ok(client.search(query).await)
}

#[tauri::command]
async fn move_search_selection(client: State<'_, NativeClient>, step: InboxStep) -> CommandResult {
    Ok(client.move_search_selection(step).await)
}

#[tauri::command]
async fn open_search_selection(client: State<'_, NativeClient>) -> CommandResult {
    Ok(client.open_search_selection().await)
}

#[tauri::command]
async fn open_search_hit(client: State<'_, NativeClient>, message_id: MessageId) -> CommandResult {
    Ok(client.open_search_hit(message_id).await)
}

#[tauri::command]
async fn attach_file(
    client: State<'_, NativeClient>,
    request: tauri::ipc::Request<'_>,
) -> CommandResult {
    let InvokeBody::Raw(bytes) = request.body() else {
        return Err("attach_file expects a raw body".to_owned());
    };
    let room_id = request
        .headers()
        .get(ROOM_ID_HEADER)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<RoomId>().ok())
        .ok_or_else(|| "attach_file is missing a room id".to_owned())?;
    let file_name = request
        .headers()
        .get(FILE_NAME_HEADER)
        .and_then(|value| value.to_str().ok())
        .map(|value| percent_decode_str(value).decode_utf8_lossy().into_owned())
        .ok_or_else(|| "attach_file is missing a file name".to_owned())?;
    Ok(client.attach_file(room_id, file_name, bytes.clone()).await)
}

#[tauri::command]
async fn discard_draft(client: State<'_, NativeClient>, draft_id: DraftId) -> CommandResult {
    Ok(client.discard_draft(draft_id).await)
}

#[tauri::command]
async fn retry_draft(client: State<'_, NativeClient>, draft_id: DraftId) -> CommandResult {
    Ok(client.retry_draft(draft_id).await)
}

#[tauri::command]
async fn set_pin(
    client: State<'_, NativeClient>,
    room_id: RoomId,
    message_id: MessageId,
    pinned: bool,
) -> CommandResult {
    Ok(client.set_pin(room_id, message_id, pinned).await)
}

#[tauri::command]
async fn open_vault(
    client: State<'_, NativeClient>,
    room_id: RoomId,
    tab: VaultTab,
) -> CommandResult {
    Ok(client.open_vault(room_id, tab).await)
}

#[tauri::command]
async fn load_more_vault(client: State<'_, NativeClient>) -> CommandResult {
    Ok(client.load_more_vault().await)
}

#[tauri::command]
async fn close_vault(client: State<'_, NativeClient>) -> CommandResult {
    Ok(client.close_vault().await)
}

#[tauri::command]
async fn toggle_shelf(client: State<'_, NativeClient>) -> CommandResult {
    Ok(client.toggle_shelf().await)
}

#[tauri::command]
async fn update_shelf(
    client: State<'_, NativeClient>,
    room_id: RoomId,
    message_id: MessageId,
    action: ShelfAction,
    preset: Option<ReminderPreset>,
    remind_at_ms: Option<u64>,
) -> CommandResult {
    Ok(client
        .update_shelf(room_id, message_id, action, preset, remind_at_ms)
        .await)
}

#[tauri::command]
async fn dismiss_reminder(client: State<'_, NativeClient>) -> CommandResult {
    Ok(client.dismiss_reminder().await)
}

#[tauri::command]
async fn watch_snapshots(
    client: State<'_, NativeClient>,
    on_snapshot: tauri::ipc::Channel<ClientSnapshot>,
) -> Result<(), String> {
    client.forward_snapshots(move |snapshot| on_snapshot.send(snapshot).is_ok());
    Ok(())
}

pub fn run() {
    let outcome = tauri::Builder::default()
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            app.manage(NativeClient::from_env_in(data_dir)?);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            client_snapshot,
            sign_up,
            sign_in,
            sign_out,
            refresh_campuses,
            create_campus,
            select_campus,
            select_room,
            send_message,
            discard_message,
            send_reply,
            react,
            set_topic_title,
            set_thread_subscription,
            open_thread,
            close_thread,
            toggle_inbox,
            move_inbox_selection,
            archive_inbox_selection,
            start_inbox_reply,
            cancel_inbox_reply,
            toggle_search,
            search,
            move_search_selection,
            open_search_selection,
            open_search_hit,
            attach_file,
            discard_draft,
            retry_draft,
            set_pin,
            open_vault,
            load_more_vault,
            close_vault,
            toggle_shelf,
            update_shelf,
            dismiss_reminder,
            watch_snapshots
        ])
        .run(tauri::generate_context!());
    if let Err(error) = outcome {
        eprintln!("pulse desktop failed: {error}");
    }
}
