// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Manager;

struct LogState {
  path: PathBuf,
}

#[tauri::command]
fn read_logs(state: tauri::State<LogState>) -> Result<String, String> {
  fs::read_to_string(&state.path).map_err(|e| e.to_string())
}

#[tauri::command]
fn search_logs(state: tauri::State<LogState>, query: String) -> Result<Vec<String>, String> {
  let content = fs::read_to_string(&state.path).map_err(|e| e.to_string())?;
  let results: Vec<String> = content
    .lines()
    .filter(|line| line.contains(&query))
    .map(String::from)
    .collect();
  Ok(results)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .manage(LogState {
      path: PathBuf::from("logs.txt"), // Fixed log file
    })
    .invoke_handler(tauri::generate_handler![read_logs, search_logs])
    .run(tauri::generate_context!())
    .expect("error while running Tauri application");
}
