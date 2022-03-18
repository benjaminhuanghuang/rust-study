type MyResult<T> = Result<T, Box<dyn Error>>;


fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
  match filename {
  "-" => Ok(Box::new(BufReader::new(io::stdin()))),
  _ =>
  Ok(Box::new(BufReader::new(File::open(filename)?))),
  }
}

//the file value must implement the BufRead trait
pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
  let mut num_lines = 0;
  let mut num_words = 0;
  let mut num_bytes = 0;
  let mut num_chars = 0;
  let mut line = String::new();

  loop {
    let line_bytes = file.read_line(&mut line)?;
    if line_bytes == 0 {
      break;
    }
    num_bytes += line_bytes;
    num_lines += 1;
    num_words += line.split_whitespace().count();
    num_chars += line.chars().count();
    line.clear();
  }

  Ok(FileInfo {
    num_lines,
    num_words,
    num_bytes,
    num_chars,
  })
}
/*
use std::io ::Cur sor in my test to fake a filehandle for the count function. 

A Cursor is “used with in-memory buffers, anything implementing AsRef<[u8]>, 
to allow them to implement Read and/or Write,

*/


#[cfg(test)]
mod tests {
  use super::{count, format_field, FileInfo};
  use std::io::Cursor;

  #[test]
  fn test_count() {
    let text = "I don't want the world. I just want your half.\r\n";
    let info = count(Cursor::new(text));
    assert!(info.is_ok());
    let expected = FileInfo {
      num_lines: 1,
      num_words: 10,
      num_chars: 48,
      num_bytes: 48,
    };
    assert_eq!(info.unwrap(), expected);
  }
}