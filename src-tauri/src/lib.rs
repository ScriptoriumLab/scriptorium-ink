use std::time::Duration;
use tauri::{AppHandle, Emitter};

const CANDIDATES: &[&str] = &[
    "候选词1",
    "候选词2",
    "候选词3",
    "候选词4",
    "候选词5"
];

#[tauri::command]
async fn process_input(app: AppHandle) -> Result<(), String> {
    app.emit("candidate_update", CANDIDATES).unwrap();

    Ok(())
}

#[tauri::command]
fn select_candidate(index: usize) {
    println!("Selected index: {}, and candidate is: {}", index, CANDIDATES[index]);
}

pub fn run() {
    tauri::Builder::default()
    .setup(|app| {
        let handle = app.handle().clone();
        tauri::async_runtime::spawn(async move {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let _ = process_input(handle).await;
        });

        Ok(())
    })
    .invoke_handler(tauri::generate_handler![
        select_candidate
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}