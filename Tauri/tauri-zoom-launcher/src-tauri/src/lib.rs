use serde::{Deserialize, Serialize};
use std::fs;
use std::process::{Child, Command};
use sysinfo::{Process, Signal, System};

#[cfg(target_os = "macos")]
const APPLICATION_DIRS: &[&str] = &["/Applications", "/Users/*/Applications"];

#[cfg(target_os = "windows")]
const APPLICATION_DIRS: &[&str] = &["C:\\Program Files", "C:\\Program Files (x86)"];

#[cfg(target_os = "linux")]
const APPLICATION_DIRS: &[&str] = &["/usr/bin", "/usr/local/bin", "/opt"];

#[derive(Serialize, Deserialize)]
struct CommandOutput {
  is_success: bool,
  information: Vec<String>,
}

/*
  Load the configuration file from the given path
*/
#[tauri::command]
fn load_configuration() -> CommandOutput {
  let mut result = CommandOutput {
    is_success: false,
    information: vec![],
  };
  result
    .information
    .push(format!("Run command: {}", "load_configuration"));
  result
}

/*
  Run the zoom client from installed package
*/
#[tauri::command]
fn run_from_installed() -> CommandOutput {
  let mut result = CommandOutput {
    is_success: false,
    information: vec![],
  };
  result
    .information
    .push(format!("Run command: {}", "run_from_installed"));

  result
}

/*
  Run the zoom client with local source code
*/
#[tauri::command]
fn run_with_local_source() -> CommandOutput {
  let mut result = CommandOutput {
    is_success: false,
    information: vec![],
  };
  result
    .information
    .push(format!("Run command: {}", "run_with_local_source"));

  result
}

/*
  Run the zoom client with local source and zoom bridge
*/
#[tauri::command]
fn run_with_local_source_bridge() -> CommandOutput {
  let mut result = CommandOutput {
    is_success: false,
    information: vec![],
  };
  result
    .information
    .push(format!("Run command: {}", "run_with_local_source_bridge"));

  result
}
#[tauri::command]
fn close_zoom_client() -> CommandOutput {
  let mut result = CommandOutput {
    is_success: false,
    information: vec![],
  };

  if kill_application("zoomdev.us") {
    result.is_success = true;
    result.information.push("Zoom client is closed".to_string());
  } else {
    result.is_success = false;
    result
      .information
      .push("Failed to close Zoom Client.".to_string());
  }
  result
}

fn kill_application(app_name: &str) -> bool {
  let mut system = System::new_all();
  system.refresh_all();

  for (pid, process) in system.processes() {
    if process.name() == app_name {
      println!(
        "Found process '{}' with PID {}",
        process.name().to_string_lossy().to_string(),
        pid
      );
      // Kill the process
      if !process.kill() {
        println!(
          "Failed to kill the process'{}' with PID {}",
          process.name().to_string_lossy().to_string(),
          pid
        );
        return false;
      }
    }
  }
  true
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .invoke_handler(tauri::generate_handler![
      load_configuration,
      close_zoom_client,
      run_from_installed,
      run_with_local_source,
      run_with_local_source_bridge
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
