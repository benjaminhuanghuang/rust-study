Rust macros are very different from macros in C. 
Rust macros are applied to the token tree whereas C macros are text substitution.


## Types of macros in Rust

- Declarative macros 
enable you to write something similar to a match expression that operates on the Rust code you provide as arguments. It uses the code you provide to generate code that replaces the macro invocation
```
  macro_rules! add{
  // macth like arm for macro
      ($a:expr,$b:expr)=>{
  // macro expand to this code
          {
              $a+$b
          }
      }
  }

  fn main(){
  // call to macro, $a=1 and $b=2
      add!(1,2);
  }
```
- Procedural macros 
allow you to operate on the abstract syntax tree (AST) of the Rust code it is given. A proc macro is a function from a TokenStream (or two) to another TokenStream, where the output replaces the macro invocation