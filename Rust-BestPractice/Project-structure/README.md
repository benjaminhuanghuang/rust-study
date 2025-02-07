# Manageing Projects

## Target
- Organize large programs
- Reuse code across applications
- Control code scope and privacy




## Package
当使用 Cargo 执行 new 命令创建 Rust 工程时，工程目录下会建立一个 Cargo.toml 文件。
工程的实质就是一个`package`，package必须由一个 Cargo.toml 文件来管理，该文件描述了package的信息以及依赖项。


## Crate
Package store `crates`.
We can define crates in cargo.toml 

If **main.rs** defined in the source directory then
a binary `crate` with the same name as the package will be automatically created and
main.rs will be the `crate root` 
The `crate root` is the source file that the rust compiler starts at when building your crate

If **lib.rs** defined in the root of the source directory, Rust will create a library crate with the same name as the package. lib.rs will be the crate root.

- Package must have at least 1 crate (bin or lib).
- Package can have 0 or 1 lib crate, and any number of bin crate.

Each file in `src/bin` folder respresents a binary crate

## Module
```
  mod front_of_house {
    mod hosting {
      fn add_to_list() {

      }
    }

    mod  serving {

    }
  }
```


## Path
```
  mod front_of_house {
      pub mod hosting {
          pub fn add_to_waitlist() {}
      }
  }

  pub fn eat_at_restaurant() {
      // Absolute path
      crate::front_of_house::hosting::add_to_waitlist();

      // Relative path : start from current module.
      front_of_house::hosting::add_to_waitlist();
  }

```
super and self
```
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use self::front_of_house::hosting;
```

## use : bringing path to scope
```
  mod front_of_house {
      pub mod hosting {
          pub fn add_to_waitlist() {}
      }
  }
  // bring hosting inside the front_of_house
  // front_of_house::hosting::add_to_waitlist();
  use crate::front_of_house::hosting;

  pub fn eat_at_restaurant() {
      hosting::add_to_waitlist();
      hosting::add_to_waitlist();
      hosting::add_to_waitlist();
  }
```


## re-exporting
```
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;
```
By using pub use, external code can now call the add_to_waitlist function using hosting::add_to_waitlist.


## Separte module in different file
src/front_of_house.rs
```
pub mod hosting;
```
src/front_of_house/hosting.rs
```
pub fn add_to_waitlist() {}  
```
