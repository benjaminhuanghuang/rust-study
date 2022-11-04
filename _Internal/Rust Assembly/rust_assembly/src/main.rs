#![feature(start)]

#[start]
#[no_mangle]
fn boost(argc: isize, argv: *const *const u8) -> isize {
  mangled();
  try_variable();
  try_match();
  try_match_with_arg(1);
  try_so_many_arg(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
  try_str_arg("wow");
  0
}

fn mangled() {}

#[no_mangle]
fn try_variable() {
  let foo = 1;
  let bar = 2;
  let mut bar = 3;
  bar += 1;
  let foo = 5;
}

#[no_mangle]
fn try_match() {
  let mut foo = 1;
  match foo {
    1 => {
      foo = 2;
    }
    _ => {
      foo = 3;
    }
  }
}

#[no_mangle]
fn try_match_with_arg(foo: i32) -> i32 {
  match foo {
    1 => 0,
    2 => 1,
    _ => 2,
  }
}

#[no_mangle]
fn try_so_many_arg(
  a: i32,
  b: i32,
  c: i32,
  d: i32,
  e: i32,
  f: i32,
  g: i32,
  h: i32,
  i: i32,
  j: i32,
  k: i32,
  l: i32,
) {
  let a = a + 1;
  let b = b + 2;
}

#[no_mangle]
fn try_str_arg(s: &str) {
  let _ = s.len();
}
