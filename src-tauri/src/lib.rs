use std::sync::{Mutex};
use tauri::{ipc::Channel, AppHandle, Manager, State};
use serde::Serialize;
use tokio::time::{sleep, Duration};

const CANDIDATES: &[&str] = &[
    "候选词1",
    "候选词2",
    "候选词3",
    "候选词4",
    "候选词5"
];

#[derive(Clone, Serialize)]
struct CandidateUpdate {
    candidates: Vec<&'static str>,
}

// 用于存储前端传入的 Channel 句柄
struct CandidateChannel(Mutex<Option<Channel<CandidateUpdate>>>);

#[tauri::command]
// 前端调用一次，用于注册监听通道
fn register_candidate_channel(channel: Channel<CandidateUpdate>, state: State<CandidateChannel>) {
    *state.0.lock().unwrap() = Some(channel);
}

#[tauri::command]
fn select_candidate(index: usize) {
    println!("Selected index: {}, and candidate is: {}", index, CANDIDATES[index]);
}

// 该函数在收到 modian-win 的候选更新时被调用
fn on_candidates_from_core(app: &AppHandle, update: CandidateUpdate) {
    if let Some(ref chan) = *app.state::<CandidateChannel>().0.lock().unwrap() {
        chan.send(update).unwrap();
    }
}

pub fn run() {
    tauri::Builder::default()
        .manage(CandidateChannel(Mutex::new(None)))
        .setup(|app| {
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                sleep(Duration::from_secs(3)).await;
                let update = CandidateUpdate {
                    candidates: CANDIDATES.to_vec(),
                };
                on_candidates_from_core(&app_handle, update);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            register_candidate_channel,
            select_candidate
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}