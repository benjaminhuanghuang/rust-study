## Modules
Provide isolated `namespace` to control `scope` and `privacy`
For example, put functions in module file_io and network_io

Modules can contain:
- Functions
- Macros
- Types
- Traits
- Constants
- Modules

Define modules
- inline
```
  mod <name> {

  }
```
- Separate file


- Privacy
All items in a module (sub module and funcitons) are private by default

Parent modules `can not` use private items in a child module

Child modules `can` use item from parent modules


When made public all enum variants are all public.

To Structure, need to add pub to all fields.


## Path
- Reference items within the module system
- can be relative or absolute

Abolute path
```
  crate::
  or
  <creat-name>::
```
Relatvie Path, starts from the current module. Can start from `self` or `super` or module name

尽量使用 relative path，以免rename module影响很多代码

- Bring path into scope: `use` statment  
```
  use <a>::<b>
```



## Crate

Crate is a collection of Rust source code files.

- Binary crates 
Binary crates compile to produce an executable program

- Library crates 
Library crates contain code for other programs to use
The code in libs can be accesse in whole crate
```
  <catr>::<funcation>()
```

By default, all the variables and functions in a module are private, which means they are accessible only to other code within the same module.


- Coure external crates


## Package
- Contains many crates and one library crate.
- The root directory of a project is the package name by default.
- each file within the /src/bin folder represent additional binary crates.





