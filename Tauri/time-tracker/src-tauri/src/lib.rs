use serde::{Deserialize, Serialize};
use sysinfo::{Process, ProcessStatus, System};

#[cfg(target_os = "macos")]
const APPLICATION_DIRS: &[&str] = &["/Applications", "/Users/*/Applications"];

#[cfg(target_os = "windows")]
const APPLICATION_DIRS: &[&str] = &["C:\\Program Files", "C:\\Program Files (x86)"];

#[cfg(target_os = "linux")]
const APPLICATION_DIRS: &[&str] = &["/usr/bin", "/usr/local/bin", "/opt"];

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}! You've been greeted from Rust!", name)
}

//
#[derive(Serialize, Deserialize)]
struct AppInfo {
  id: String,
  name: String,
  running_time_formatted: String,
  memory_in_bytes: u64,
}
fn is_valid(process: &Process) -> bool {
  let helper_keywords = vec!["helper", "service", "daemon", "agent", "."];
  if let Some(exe_path) = process.exe().unwrap().to_str() {
    let is_in_app_dir = APPLICATION_DIRS.iter().any(|dir| exe_path.starts_with(dir));
    let is_helper = helper_keywords.iter().any(|keyword| {
      process
        .name()
        .to_string_lossy()
        .to_lowercase()
        .contains(keyword)
    });
    process.status() == ProcessStatus::Run && is_in_app_dir && !is_helper
  } else {
    false
  }
}

fn format_running_time(secconds: u64) -> String {
  let days = secconds / 86400;
  let hours = (secconds % 86400) / 3600;
  let minutes = (secconds % 3600) / 60;
  let seconds = secconds % 60;

  format!(
    "{:02} Days:{:02} Hours:{:02} Minutes: {:02} Seconds ",
    days, hours, minutes, seconds
  )
}

#[tauri::command]
fn max_running_process() -> Option<AppInfo> {
  let mut sys = System::new_all();
  sys.refresh_all();

  sys
    .processes()
    .iter()
    .filter(|(_, process)| is_valid(process))
    .max_by_key(|(_, process)| process.run_time())
    .map(|(id, process)| AppInfo {
      id: id.to_string(),
      name: process.name().to_string_lossy().to_string(),
      running_time_formatted: format_running_time(process.run_time()),
      memory_in_bytes: process.memory(),
    })
}

#[tauri::command]
fn max_memory() -> Option<AppInfo> {
  let mut sys = System::new_all();
  sys.refresh_all();

  sys
    .processes()
    .iter()
    .filter(|(_, process)| is_valid(process))
    .max_by_key(|(_, process)| process.memory())
    .map(|(id, process)| AppInfo {
      id: id.to_string(),
      name: process.name().to_string_lossy().to_string(),
      running_time_formatted: format_running_time(process.run_time()),
      memory_in_bytes: process.memory(),
    })
}

#[tauri::command]
fn list_process() -> Vec<AppInfo> {
  let mut sys = System::new_all();
  sys.refresh_all();

  let mut processes: Vec<AppInfo> = sys
    .processes()
    .iter()
    .filter(|(_, process)| is_valid(process))
    .map(|(id, process)| AppInfo {
      id: id.to_string(),
      name: process.name().to_string_lossy().to_string(),
      running_time_formatted: format_running_time(process.run_time()),
      memory_in_bytes: process.memory(),
    })
    .collect();
  processes.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
  processes
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .invoke_handler(tauri::generate_handler![
      greet,
      list_process,
      max_memory,
      max_running_process
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
