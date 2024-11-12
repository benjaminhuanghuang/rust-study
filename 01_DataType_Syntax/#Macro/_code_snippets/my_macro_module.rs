/*

In order to use a macro outside of its module, you need to use #[macro_use]
to the module to lift the macro out into its parent.

*/
#[macro_use]
mod macros {
  macro_rules! my_macro {
    () => {
      println!("Check out my macro!");
    };
  }
}

fn main() {
  my_macro!();
}
