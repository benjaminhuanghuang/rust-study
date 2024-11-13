use serde::{Deserialize, Serialize};
use std::fs;
use std::process::{Child, Command};

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

fn start_application() -> std::io::Result<Child> {
  Command::new("your_application")
    .arg("your_argument") // Add any required arguments
    .spawn() // Start the application as a child process
}

fn kill_application(child: &mut Child) -> std::io::Result<()> {
  child.kill() // Sends a kill signal to the child process
}

fn restart_application(child: &mut Child) -> std::io::Result<Child> {
  // Kill the application if it's still running
  child.kill()?;
  // Wait for the process to exit completely
  child.wait()?;
  // Start the application again
  start_application()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .invoke_handler(tauri::generate_handler![
      load_configuration,
      run_from_installed,
      run_with_local_source,
      run_with_local_source_bridge
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
