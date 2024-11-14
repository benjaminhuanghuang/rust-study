use serde::{Deserialize, Serialize};
use std::process::Command;
use sysinfo::System;

/*
  Configuration:
  path of zoombridge
  path of zoom client
  path of mail client

  process API:
  .spawn() starts the command asynchronously and immediately returns a Child process handle.
  .status() or .output() wait for the command to complete,
*/
// #[cfg(target_os = "macos")]
// const APPLICATION_DIRS: &[&str] = &["/Applications", "/Users/*/Applications"];

// #[cfg(target_os = "windows")]
// const APPLICATION_DIRS: &[&str] = &["C:\\Program Files", "C:\\Program Files (x86)"];

// #[cfg(target_os = "linux")]
// const APPLICATION_DIRS: &[&str] = &["/usr/bin", "/usr/local/bin", "/opt"];

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

  kill_application("zoomdev.us");
  kill_npm_processes();
  //start_mail_client();
  /*
   Tell Zoom client to load js code from the installed package
  */
  set_user_preferences("");

  start_zoom_client();
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

  kill_application("zoomdev.us");
  kill_npm_processes();
  start_mail_client();
  /*
   modify user preferences to tell Zoom client to load local js code
  */
  set_user_preferences("http://127.0.0.1:8080");
  set_webview();
  start_zoom_client();
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

  kill_application("zoomdev.us");
  kill_npm_processes();

  start_zoom_bridge();
  /*
   modify user preferences to tell Zoom client to load local js code from the bridge
  */
  set_user_preferences("http://localhost:3000");
  start_mail_client_with_bridge();

  start_zoom_client();
  result
}

#[tauri::command]
fn read_user_preferences() -> CommandOutput {
  let mut result = CommandOutput {
    is_success: false,
    information: vec![],
  };
  result
    .information
    .push(format!("Run command: {}", "read_user_preferences"));
  //defaults read ZoomChat mail.localHtmlPath
  let output = Command::new("defaults")
    .arg("read")
    .arg("ZoomChat")
    .arg("mail.localHtmlPath")
    .output() // Executes the command, capturing output
    .expect("Failed to execute command");
  if output.status.success() {
    // Convert stdout (standard output) from bytes to a String
    let output = String::from_utf8_lossy(&output.stdout);
    result.is_success = true;
    result
      .information
      .push(format!("User preferences: {}", output));
  } else {
    result.is_success = false;
    // If there was an error, capture the stderr output
    let error = String::from_utf8_lossy(&output.stderr);
    println!("User preferences Error: {}", error);
  }
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

fn kill_npm_processes() {
  Command::new("pkill")
    .arg("-f") // Match the full command line
    .arg("npm") // Target the npm processes
    .status()
    .expect("Failed to execute pkill");
}

fn start_mail_client() {
  let mut child = Command::new("npm")
    .arg("run")
    .arg("serve:enable-proxy")
    .current_dir("/Users/BenjaminHuang/workspace/client-email")
    .spawn()
    .expect("Failed to execute command");

  let status = child.wait().expect("Failed to wait on child process");
  if !status.success() {
    println!("Failed to start mail client");
  } else {
    println!("Mail client is started");
  }
}

fn start_mail_client_with_bridge() {
  let mut child = Command::new("npm")
    .arg("run")
    .arg("serve:enable-proxy")
    .current_dir("/Users/BenjaminHuang/workspace/client-email")
    .spawn()
    .expect("Failed to execute command");

  let status = child.wait().expect("Failed to wait on child process");
  if !status.success() {
    println!("Failed to start mail client");
  }
}
/*
  Start zoom bridge
  node apps/commandline/bin/zoom-bridge.js start --port 3000
*/
fn start_zoom_bridge() {
  Command::new("node")
    .arg("apps/commandline/bin/zoom-bridge.js")
    .arg("start")
    .arg("--port")
    .arg("3000")
    .current_dir("/Users/BenjaminHuang/workspace/zoombridge")
    .spawn()
    .expect("Failed to execute command");
}

fn set_user_preferences(url: &str) {
  Command::new("defaults")
    .arg("write")
    .arg("ZoomChat")
    .arg("mail.localHtmlPath")
    .arg(url)
    .status()
    .expect("Failed to execute first command");
}

fn set_webview() {
  Command::new("defaults")
    .arg("write")
    .arg("ZoomChat")
    .arg("webview.context.menu")
    .arg("true")
    .status()
    .expect("Failed to execute first command");
}

fn start_zoom_client() {
  Command::new("open")
    .arg("-a") //-a flag specifies the name of the application to open
    .arg("/Applications/zoomdev.us.app")
    .spawn()
    .expect("Failed to execute command");
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
      run_with_local_source_bridge,
      read_user_preferences
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
