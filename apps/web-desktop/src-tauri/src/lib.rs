use pulse_core::native::NativeClient;
use pulse_protocol::{
    CampusId, ClientNonce, ClientSnapshot, CreateCampusRequest, RoomId, SignInRequest,
    SignUpRequest,
};
use tauri::{Manager, State};

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
            app.manage(NativeClient::from_env()?);
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
            watch_snapshots
        ])
        .run(tauri::generate_context!());
    if let Err(error) = outcome {
        eprintln!("pulse desktop failed: {error}");
    }
}
