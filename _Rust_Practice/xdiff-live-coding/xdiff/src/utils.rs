use anyhow::Result;
use console::{style, Style};
use similar::{ChangeTag, TextDiff};
use std::fmt;
use std::fmt::Write as _;
use std::io::Write as _;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

struct Line(Option<usize>);

impl fmt::Display for Line {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.0 {
      None => write!(f, "    "),
      Some(idx) => write!(f, "{:<4}", idx + 1),
    }
  }
}

pub fn diff_text(text1: &str, text2: &str) -> Result<String> {
  let mut output = String::new();
  let diff = TextDiff::from_lines(text1, text2);

  for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
    if idx > 0 {
      writeln!(&mut output, "{:-^1$}", "-", 80)?;
    }
    for op in group {
      for change in diff.iter_inline_changes(op) {
        let (sign, s) = match change.tag() {
          ChangeTag::Delete => ("-", Style::new().red()),
          ChangeTag::Insert => ("+", Style::new().green()),
          ChangeTag::Equal => (" ", Style::new().dim()),
        };
        write!(
          &mut output,
          "{}{} |{}",
          style(Line(change.old_index())).dim(),
          style(Line(change.new_index())).dim(),
          s.apply_to(sign).bold(),
        )?;
        for (emphasized, value) in change.iter_strings_lossy() {
          if emphasized {
            write!(&mut output, "{}", s.apply_to(value).underlined().on_black())?;
          } else {
            write!(&mut output, "{}", s.apply_to(value))?;
          }
        }
        if change.missing_newline() {
          writeln!(&mut output)?;
        }
      }
    }
  }

  Ok(output)
}

pub fn highlight_text(text: &str, extension: &str, theme: Option<&str>) -> Result<String> {
  // Load these once at the start of your program
  let ps = SyntaxSet::load_defaults_newlines();
  let ts = ThemeSet::load_defaults();

  let syntax = if let Some(s) = ps.find_syntax_by_extension(extension) {
    s
  } else {
    ps.find_syntax_plain_text()
  };
  let mut h = HighlightLines::new(syntax, &ts.themes[theme.unwrap_or("base16-ocean.dark")]);

  let mut output = String::new();

  for line in LinesWithEndings::from(text) {
    let ranges = h.highlight_line(line, &ps).unwrap();
    let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
    write!(&mut output, "{}", escaped)?;
  }

  Ok(output)
}

pub fn process_error_output(result: Result<()>) -> Result<()> {
  match result {
    Ok(_) => {}
    Err(e) => {
      let stderr = std::io::stderr();
      let mut stderr = stderr.lock();
      if atty::is(atty::Stream::Stderr) {
        let s = Style::new().red();
        write!(stderr, "{}", s.apply_to(format!("{:?}", e)))?;
      } else {
        write!(stderr, "{:?}", e)?;
      }
    }
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use serde_json::json;

  use super::*;

  #[test]
  fn diff_text_should_work() {
    let text1 = "foo\nbar";
    let text2 = "foo\nbaz";
    let expected = include_str!("../fixtures/diff1.txt");
    assert_eq!(diff_text(text1, text2).unwrap(), expected);
  }

  #[test]
  fn highlight_text_should_work() {
    let v = json!({
        "foo": "bar",
        "baz": "qux"
    });
    let text = serde_json::to_string_pretty(&v).unwrap();
    let expected = include_str!("../fixtures/highlight1.txt");
    assert_eq!(highlight_text(&text, "json", None).unwrap(), expected);
  }
}
