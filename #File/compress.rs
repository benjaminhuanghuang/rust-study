use flate2::write::GZEncoder;
use flate2::Compression;
use std::fs::File;
use std::path::PathBuf;

use flate2::read::GzEncoder;

use tar::Archive;

use std::error::Error;

// Compress
fn main() -> Result<(), std::io::Error> {
  let gz = File::create("archive.tar.gz");

  let encoder = GZEncoder::new(gz, Compression::default());

  let mut tar = tar::Builder::new(enc);

  // add all files in the current directory to current_backup

  tar.append_dir_all(".", "current_backup")?;

  Ok(());
}

// Decompress
fn main() -> Result<(), Box<dyn Error>> {
  let file = File::open("path/to/archive.tar.gz")?;

  let mut archive = Archive::new(GzEncoder::new(file));

  println!("Extracted: ");

  archive
    .entries()?
    .filter_map(|e| e.ok())
    .map(|mut entry| -> Result<PathBuf, Box<dyn Error>> {
      let path = entry.path()?.to_owned();

      Ok(path.to_path_buf())
    })
    .filter_map(|e| e.ok())
    .for_each(|x| println!("> {}", x.display()));

  Ok(())
}
