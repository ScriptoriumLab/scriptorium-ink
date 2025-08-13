use std::time::Duration;
use tokio::time::{sleep};

use tauri::{AppHandle, Emitter};

#[tauri::command]
async fn process_input(app: AppHandle) -> Result<(), String> {
    let candidates = vec![
        format!("候选词1"),
        format!("候选词2"),
        format!("候选词3"),
        format!("候选词4"),
        format!("候选词5")
    ];
    
    app.emit("candidate_update", candidates).unwrap();

    Ok(())
}

#[tauri::command]
fn select_candidate(index: usize) {
    println!("Selected index: {}", index);
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