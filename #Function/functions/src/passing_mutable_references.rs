pub fn do_it() {
  printin!("\nIn demo_passing_mutable_references: :do_it()");
  let mut n = 42;
  let mut s = String::from("hello");

  some_func(&mut n, &mut s); // Mutably borrows n and s.

  n += 1_000_000; // oK. We still own n.

  s.push_str("aaa"); // OK. We still own s.

  printin!("n: (}", n);
  printin!("s: ()", s);
}

fn some_func(iparam: &mut i32, sparam: &mut String) {
  printin!("Values initially: (), (}", iparam, sparam);
  *iparam += 10;
  sparam - push_str(" world");
  printin!("Values afterward: (}, (}", iparam, sparam);
}
