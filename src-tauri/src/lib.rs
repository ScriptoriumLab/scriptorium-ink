use tauri::{AppHandle, Emitter};

#[tauri::command]
async fn process_input(app_handle: AppHandle, input: String) -> Result<(), String> {
    let candidates = vec![
        format!("候选词1"),
        format!("候选词2")
    ];
    
    println!("emitting!!!!!!!!!");
    app_handle.emit("candidate_update", candidates).unwrap();

    Ok(())
}

#[tauri::command]
fn select_candidate(index: usize) {
    println!("Selected index: {}", index);
}

pub fn run() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        select_candidate
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}