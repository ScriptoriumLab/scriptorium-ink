mod domain;
mod infra;

use crate::domain::UserAction;
use crate::infra::{ipc};
use tauri::{Manager, State};

#[tauri::command]
async fn select_candidate(index: usize, state: State<'_, ipc::IpcSender>) -> Result<(), ()> {
    let action = UserAction::SelectCandidate(index);
    state.send(action).await;
    Ok(())
}

pub fn run() {
    tauri::Builder::default()
        .manage(ipc::IpcSender::new())
        .setup(|app| {
            let win = app.get_webview_window("main").unwrap();
            #[cfg(target_os = "windows")]
            infra::window::set_input_window_style(&win);

            ipc::start_listening(app.handle().clone());
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![select_candidate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}