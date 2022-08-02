fn main() {
  let args = std::env::args().collect::<Vec<String>>();

  if args.len() != 2 {
    eprintln!("stat: missing operand");
    eprintln!("Try 'stat --help' for more information");
    return;
  }

  match args[1].as_str() {
    "--version" => {
      println!("stat (GNU coreutils) rewritten in Rust");
      println!("source code: https://github.com/pymongo/linux_commands_rewritten_in_rust");
    }
    "--help" => {
      println!("help doc is working in progress");
    }
    filename => {
      my_stat(filename);
    }
  }
}

fn my_stat(filename: &str) {
  let filename_with_nul = format!("{}\0", filename);
  let mut file_stat = unsafe { std::mem::zeroed() };
  syscall!(stat(filename_with_nul.as_ptr().cast(), &mut file_stat));
  println!("  File: {}", filename);
  println!(
    "  Size: {:<15} Blocks: {:<10} IO Block: {:<6} {}",
    file_stat.st_size,
    file_stat.st_blocks,
    file_stat.st_blksize,
    get_filetype(file_stat.st_mode)
  );
  println!("Device: ");
  println!("Access: ");

  let access_time = format_timestamp_with_nanosecond(file_stat.st_atime, file_stat.st_atime_nsec);
  let modify_time = format_timestamp_with_nanosecond(file_stat.st_mtime, file_stat.st_mtime_nsec);
  println!("Access: {}", access_time);
  println!("Modify: {}", modify_time);
  println!("Change: {}", modify_time);
  // FIXME `/dev/console` create_time should be null
  println!(" Birth: {}", access_time);
}
