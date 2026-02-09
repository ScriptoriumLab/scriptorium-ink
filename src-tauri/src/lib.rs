mod domain;
mod infra;

use crate::domain::RenderState;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager};
use tokio::time::{sleep, Duration};

// 模拟 Inkstone 的行为 (Mock Driver)
fn start_mock_driver(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut index = 0;
        let candidates = vec![
            "你好", "拟好", "泥壕", "逆号", "尼耗"
        ];
        
        loop {
            sleep(Duration::from_millis(1000)).await;
            
            // 模拟：每秒钟光标移动一次
            index = (index + 1) % candidates.len();

            let state = RenderState {
                visible: true,
                candidates: candidates.iter().map(|s| s.to_string()).collect(),
                highlight_index: index, // 告诉前端高亮哪一个
                page_index: 0,
                total_pages: 1,
            };

            // 🔥 核心：发送事件给前端
            // 以后这里会替换成从 Pipe 读取数据
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