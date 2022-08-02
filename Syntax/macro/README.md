Rust macros are very different from macros in C. 
Rust macros are applied to the token tree whereas C macros are text substitution.


## Types of macros in Rust

Declarative macros 匹配对应模式然后以另一部分代码替换当前代码 

Procedural macros 接收代码(TokenStream)为输入，在这些代码上进行操作，然后产生另一些代码(TokenStream)为输出。

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

  - Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
  - Attribute-like macros that define custom attributes usable on any item
  - Function-like macros that look like function calls but operate on the tokens specified as their argument