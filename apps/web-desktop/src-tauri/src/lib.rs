use pulse_protocol::ClientSnapshot;

#[tauri::command]
async fn initial_snapshot() -> ClientSnapshot {
    pulse_core::initial_snapshot()
}

pub fn run() {
    let outcome = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![initial_snapshot])
        .run(tauri::generate_context!());
    if let Err(error) = outcome {
        eprintln!("pulse desktop failed: {error}");
    }
}
