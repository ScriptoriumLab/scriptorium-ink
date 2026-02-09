use crate::domain::{RenderState, UserAction};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager, WebviewWindow};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, WriteHalf};
use tokio::net::windows::named_pipe::{ClientOptions, NamedPipeClient};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    ShowWindow, SW_HIDE, SW_SHOWNOACTIVATE
};

const PIPE_NAME: &str = r"\\.\pipe\modian_ui_protocol_pipe";

pub struct IpcSender(pub Arc<Mutex<Option<WriteHalf<NamedPipeClient>>>>);

impl IpcSender {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(None)))
    }

    pub async fn send(&self, action: UserAction) {
        let mut guard = self.0.lock().await;
        if let Some(writer) = guard.as_mut() {
            // 序列化成 JSON
            if let Ok(json) = serde_json::to_string(&action) {
                // 发送数据，记得加换行符，因为我们约定是 Line-Based JSON
                let _ = writer.write_all(format!("{}\n", json).as_bytes()).await;
                let _ = writer.flush().await; // 确保刷入管道
            }
        } else {
            println!("IPC Warning: Trying to send action but pipe is not connected.");
        }
    }
}

pub fn start_listening(app: AppHandle) {
    let app_handle = app.clone();

    tauri::async_runtime::spawn(async move {
        loop {
            println!("IPC: Connecting to {}...", PIPE_NAME);
            
            match ClientOptions::new()
                .read(true)
                .write(true)
                .open(PIPE_NAME) {
                    Ok(client) => {
                        println!("IPC: Connected!");
                        handle_connection(client, &app_handle).await;
                    }
                    Err(_) => {
                        sleep(Duration::from_millis(1000)).await;
                    }
                }
        }
    });
}

fn set_window_visibility<R: tauri::Runtime>(window: &WebviewWindow<R>, visible: bool) {
    let hwnd = window.hwnd().unwrap().0;
    let hwnd = HWND(hwnd as _);

    unsafe {
        if visible {
            ShowWindow(hwnd, SW_SHOWNOACTIVATE);
        } else {
            ShowWindow(hwnd, SW_HIDE);
        }
    }
}

async fn handle_connection(client: NamedPipeClient, app: &AppHandle) {
    let (reader, writer) = tokio::io::split(client);

    {
        let state = app.state::<IpcSender>();
        *state.0.lock().await = Some(writer);
    }

    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        line.clear();
        match buf_reader.read_line(&mut line).await {
            Ok(0) => {
                println!("IPC: Server closed connection (EOF).");
                break;
            }
            Ok(_) => {
                if let Ok(state) = serde_json::from_str::<RenderState>(&line) {
                    if let Some(win) = app.get_webview_window("main") {
                        let _ = win.set_position(tauri::Position::Logical(
                            tauri::LogicalPosition::new(state.x, state.y)
                        ));

                        set_window_visibility(&win, state.visible);
                    }

                    let _ = app.emit("render_update", state);
                } else {
                    println!("IPC Error: Failed to parse JSON: {}", line);
                }
            }
            Err(e) => {
                println!("IPC Error: Read failed: {}", e);
                break;
            }
        }
    }

    {
        let state = app.state::<IpcSender>();
        *state.0.lock().await = None;
    }
}