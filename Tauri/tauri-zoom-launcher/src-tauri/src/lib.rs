use serde::{Deserialize, Serialize};
use std::process::Command;
use std::thread;
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
    is_success: true,
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
    is_success: true,
    information: vec![],
  };
  result
    .information
    .push(format!("Run command: {}", "run_from_installed"));

  kill_application("zoomdev.us");
  kill_node_processes();
  //start_mail_client();
  /*
   Tell Zoom client to load js code from the installed package
  */
  set_user_preferences("ZoomChat", "mail.localHtmlPath", "");

  start_zoom_client();
  result
}

/*
  Run the zoom client with local source code
*/
#[tauri::command]
fn run_with_local_source() -> CommandOutput {
  let mut result = CommandOutput {
    is_success: true,
    information: vec![],
  };
  result
    .information
    .push(format!("Run command: {}", "run_with_local_source"));

  kill_application("zoomdev.us");
  kill_node_processes();

  start_mail_client();
  /*
   modify user preferences to tell Zoom client to load local js code
  */
  set_user_preferences("ZoomChat", "mail.localHtmlPath", "http://127.0.0.1:8080");
  set_user_preferences("ZoomChat", "webview.context.menu", "true");

  start_zoom_client();
  result
}

/*
  Run the zoom client with local source and zoom bridge
*/
#[tauri::command]
fn run_with_local_source_bridge() -> CommandOutput {
  let mut result = CommandOutput {
    is_success: true,
    information: vec![],
  };
  result
    .information
    .push(format!("Run command: {}", "run_with_local_source_bridge"));

  kill_application("zoomdev.us");
  kill_node_processes();

  start_zoom_bridge();
  /*
   modify user preferences to tell Zoom client to load local js code from the bridge
  */
  set_user_preferences("ZoomChat", "mail.localHtmlPath", "http://localhost:3000");
  start_mail_client_with_bridge();

  start_zoom_client();

  open_chrome("http://localhost:8080");
  result
}

#[tauri::command]
fn get_user_preferences() -> CommandOutput {
  let mut result = CommandOutput {
    is_success: true,
    information: vec![],
  };
  result
    .information
    .push(format!("Run command: {}", "read_user_preferences"));

  match read_user_preferences("ZoomChat", "mail.localHtmlPath") {
    Some(value) => {
      result.is_success = true;
      result
        .information
        .push(format!("ZoomChat mail.localHtmlPath: {}", value));
    }
    None => {
      result.is_success = false;
      result
        .information
        .push("Failed to read user preferences".to_string());
    }
  }
  result
}

#[tauri::command]
fn close_zoom_client() -> CommandOutput {
  let mut result = CommandOutput {
    is_success: true,
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

fn kill_node_processes() {
  Command::new("pkill")
    .arg("-f") // Match the full command line
    .arg("node") // Target the npm processes
    .status()
    .expect("Failed to execute pkill");
}

fn start_mail_client() {
  thread::spawn(|| {
    let result = Command::new("npm")
      .arg("run")
      .arg("serve")
      .current_dir("/Users/BenjaminHuang/workspace/client-email")
      .spawn();

    match result {
      Ok(mut child) => {
        println!("Successfully started `npm run` in a new thread.");
        let _ = child.wait(); // Wait for the child process to complete if needed
      }
      Err(e) => eprintln!("Failed to start `npm run`: {}", e),
    }
  });
}

fn start_mail_client_with_bridge() {
  thread::spawn(|| {
    let result = Command::new("npm")
      .arg("run")
      .arg("serve:enable-proxy")
      .current_dir("/Users/BenjaminHuang/workspace/client-email")
      .spawn();

    match result {
      Ok(mut child) => {
        println!("Successfully started `npm run` in a new thread.");
        let _ = child.wait(); // Wait for the child process to complete if needed
      }
      Err(e) => eprintln!("Failed to start `npm run`: {}", e),
    }
  });
}
/*
  Start zoom bridge
  node apps/commandline/bin/zoom-bridge.js start --port 3000
*/
fn start_zoom_bridge() {
  thread::spawn(|| {
    let result = Command::new("node")
      .arg("apps/commandline/bin/zoom-bridge.js")
      .arg("start")
      .arg("--port")
      .arg("3000")
      .current_dir("/Users/BenjaminHuang/workspace/zoombridge")
      .spawn();

    match result {
      Ok(mut child) => {
        println!("Successfully started `npm run` in a new thread.");
        let _ = child.wait(); // Wait for the child process to complete if needed
      }
      Err(e) => eprintln!("Failed to start `npm run`: {}", e),
    }
  });
}

fn set_user_preferences(domain: &str, key: &str, value: &str) {
  Command::new("defaults")
    .arg("write")
    .arg(domain)
    .arg(key)
    .arg(value)
    .status()
    .expect("Failed to execute first command");
}

fn read_user_preferences(domain: &str, key: &str) -> Option<String> {
  //defaults read ZoomChat mail.localHtmlPath
  let output = Command::new("defaults")
    .arg("read")
    .arg(domain)
    .arg(key)
    .output() // Executes the command, capturing output
    .expect("Failed to execute command");

  if output.status.success() {
    // Convert stdout (standard output) from bytes to a String
    Some(String::from_utf8_lossy(&output.stdout).to_string())
  } else {
    None
  }
}

fn start_zoom_client() {
  Command::new("open")
    .arg("-a") //-a flag specifies the name of the application to open
    .arg("/Applications/zoomdev.us.app")
    .spawn()
    .expect("Failed to execute command");
}

fn open_chrome(url: &str) {
  let result = if cfg!(target_os = "macos") {
    Command::new("open")
      .arg("-a")
      .arg("Google Chrome")
      .arg(url)
      .spawn()
  } else if cfg!(target_os = "windows") {
    Command::new("cmd")
      .args(&["/C", "start", "chrome", url])
      .spawn()
  } else if cfg!(target_os = "linux") {
    Command::new("google-chrome").arg(url).spawn()
  } else {
    Err(std::io::Error::new(
      std::io::ErrorKind::Other,
      "Unsupported platform",
    ))
  };

  match result {
    Ok(_) => println!("Opened Chrome with URL: {}", url),
    Err(e) => eprintln!("Failed to open Chrome: {}", e),
  }
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
      get_user_preferences
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
