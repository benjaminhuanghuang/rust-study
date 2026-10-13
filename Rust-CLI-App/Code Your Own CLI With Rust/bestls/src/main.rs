use chrono::{DateTime, Utc};
use clap::Parser;
use owo_colors::OwoColorize;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use strum::Display;
use tabled::{
  Table, Tabled,
  settings::{Color, Style, object::Columns, object::Rows},
};

// Command line argument parser definition
#[derive(Parser)]
#[command(version, about, long_about = "best ls command ever")]
struct Cli {
  /// Target path (defaults to current directory if omitted)
  path: Option<PathBuf>,

  /// Output results in JSON format
  #[arg(short, long)]
  json: bool,
}

// Enum representing the entry type (File or Directory)
#[derive(Debug, Serialize, Display, Tabled)]
enum EntryType {
  File,
  Directory,
}

// Struct holding file entry metadata
#[derive(Debug, Serialize, Tabled)]
struct FileEntry {
  #[tabled(rename = "Name")]
  name: String,

  #[tabled(rename = "Type")]
  e_type: EntryType,

  #[tabled(rename = "Size (B)")]
  length: u64,

  #[tabled(rename = "Modified")]
  modified: String,
}

// Extract and map file metadata for a given path
fn map_data(path: &PathBuf, file_name: String) -> Option<FileEntry> {
  if let Ok(meta) = fs::metadata(path) {
    let e_type = if meta.is_dir() {
      EntryType::Directory
    } else {
      EntryType::File
    };

    let length = meta.len();

    let modified = if let Ok(mod_time) = meta.modified() {
      let datetime: DateTime<Utc> = mod_time.into();
      datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    } else {
      "".to_string()
    };

    Some(FileEntry {
      name: file_name,
      e_type,
      length,
      modified,
    })
  } else {
    None
  }
}

// Retrieve file entries from the specified path
fn get_files(path: &PathBuf) -> Vec<FileEntry> {
  let mut data = Vec::new();

  if let Ok(read_dir) = fs::read_dir(path) {
    for entry in read_dir {
      if let Ok(file) = entry {
        let file_name = file
          .file_name()
          .into_string()
          .unwrap_or_else(|_| "unknown name".to_string());

        if let Some(file_entry) = map_data(&file.path(), file_name) {
          data.push(file_entry);
        }
      }
    }
  }

  data
}

// Render and print formatted table to stdout
fn print_table(files: &[FileEntry]) {
  let mut table = Table::new(files);

  // Apply rounded border style
  table.with(Style::rounded());

  // Customize header and column colors
  table.modify(Columns::single(0), Color::FG_BRIGHT_CYAN);
  table.modify(Columns::single(1), Color::FG_BRIGHT_MAGENTA);
  table.modify(Columns::single(2), Color::FG_BRIGHT_YELLOW);
  table.modify(Rows::single(0), Color::FG_BRIGHT_GREEN);

  println!("{}", table);
}

fn main() {
  let cli = Cli::parse();

  // Default to current directory "." if no path is provided
  let path = cli.path.unwrap_or_else(|| PathBuf::from("."));

  // Check if the path exists
  if !path.exists() {
    println!("{}", "path does not exist".red());
    return;
  }

  let files = get_files(&path);

  // Output as JSON or styled terminal table based on flags
  if cli.json {
    let json_str =
      serde_json::to_string_pretty(&files).unwrap_or_else(|_| "cannot parse JSON".to_string());
    println!("{}", json_str);
  } else {
    print_table(&files);
  }
}
