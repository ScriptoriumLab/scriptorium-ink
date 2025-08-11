// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn get_candidates(input: String) -> Vec<String> {
  // 实际调用输入法引擎获取候选词
  vec!["候选词1".into(), "候选词2".into()]
}

#[tauri::command]
fn select_candidate(index: usize) {
  // 将选择结果插入到当前输入位置
  println!("Selected index: {}", index);
}

fn main() {
    modian_ui_lib::run()
}
