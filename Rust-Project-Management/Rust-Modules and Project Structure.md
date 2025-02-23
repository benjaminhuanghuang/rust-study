# Rust — Modules and Project Structure

https://medium.com/codex/rust-modules-and-project-structure-832404a33e2e#:~:text=Modules%20%E2%80%94%20Folders,modules)%20with%20one%20key%20difference.&text=When%20trying%20to%20import%20a,Our%20application%20is%20building%20again.&text=If%20this%20looks%20a%20little,is%20however%2C%20a%20better%20way.

When you import a module with the mod statement, Rust automatically creates a module namespace for it (to avoid conflicts) and thus we cannot access our struct type directly.

In order to access the root of that module tree, you can always use the crate:: prefix.

You must use mod to include a module (file or folder) into your application.
The use keyword is a convenience to map a fully qualified type name to just it’s type name (you can even rename types, but that’s for another post).

## Modules — Visibility

Everything inside a module (ie, a file or subfolder within the /src folder) can access anything else within that module.
Everything outside a module can only access public members of that module.
private functions within a module are still accessible for tests within that module (idiomatic Rust keeps unit tests within the module).

## Modules — Folders

```rs
mod foo;                   <-- Change the module to match the folder
use crate::foo::MyStruct;  <-- Update the namespace to 'foo'
fn main() {
  let _ms = MyStruct {};
}
```

When trying to import a module defined as a folder, we use the folder name (as we did for the file based module previously) but Rust expects a file named mod.rs exists within the folder.

we can simply rename our my_struct.rs to mod.rs
