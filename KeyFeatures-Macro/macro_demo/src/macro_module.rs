// This brings all macros defined in the `macros` module into scope.
#[macro_use]
pub mod macros {
  macro_rules! my_macro {
    () => {
      println!("Check out my macro!");
    };
  }
}

fn main() {
  my_macro!();
}
