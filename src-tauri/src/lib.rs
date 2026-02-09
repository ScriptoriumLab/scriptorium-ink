mod domain;
mod infra;

use crate::domain::RenderState;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager};
use tokio::time::{sleep, Duration};

fn start_mock_driver(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut index = 0;
        let candidates = vec![
            "你好", "拟好", "泥壕", "逆号", "尼耗"
        ];

        let mut mock_x = 500.0;
        let mut mock_y = 300.0;
        
        loop {
            sleep(Duration::from_millis(1000)).await;
            
            index = (index + 1) % candidates.len();

            mock_x += 10.0;
            mock_y += 5.0;
            if mock_x > 800.0 { mock_x = 500.0; }

            let state = RenderState {
                visible: true,
                x: mock_x,
                y: mock_y,
                candidates: candidates.iter().map(|s| s.to_string()).collect(),
                highlight_index: index,
                page_index: 0,
                total_pages: 1,
            };

            if let Some(win) = app.get_webview_window("main") {
                let _ = win.set_position(tauri::Position::Logical(
                    tauri::LogicalPosition::new(state.x, state.y)
                ));
            }

            app.emit("render_update", state).unwrap();
        }
    });
}

#[tauri::command]
fn select_candidate(index: usize) {
    println!("Frontend User Clicked Index: {}", index);
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let win = app.get_webview_window("main").unwrap();
            #[cfg(target_os = "windows")]
            infra::window::set_input_window_style(&win);

            let handle = app.handle().clone();
            
            start_mock_driver(handle);
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![select_candidate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}