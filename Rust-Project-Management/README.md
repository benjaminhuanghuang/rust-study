# Rust Projects Management

## Target

- Organize large programs
- Reuse code across applications
- Control code scope and privacy

## Terms

- workspace: 项目复杂时，管理多个 package

- package: cargo new 命令会创建一个新项目，也是一个 package, 管理 crate

- crate: 模块的集合，编译单位，有 lib 和 bin 两种，即供别人调用，或者是一个可执行文件

- module: 用于在 crate 内组织代码. 代码多了可以对代码以 mod（文件/文件夹）为单位进行拆分
  一般来说，一个文件都会被视为一个 mod，而且 mod 可以嵌套定义。嵌套定义的 mod 既可以写在同一个文件里，也可以通过文件夹的形式来实现。

## Folder structure for Rust project

Cargo.toml 与 Cargo.lock 存储在项目的根目录中。

源代码进入 src 目录。

默认库文件是 src/lib.rs。

默认的可执行文件是 src/main.rs。

其他可执行文件可以放入 src/bin/\*.rs。

集成测试进入 tests 目录（Unit test 放入要测试的每个文件中）。

示例可执行文件放在 examples 目录中。

基准测试进入 benches 目录。

## Package

当使用 Cargo 执行 new 命令创建 Rust 工程时，工程目录下会建立一个 Cargo.toml 文件。
工程的实质就是一个`package`，package 必须由一个 Cargo.toml 文件来管理，该文件描述了 package 的信息以及依赖项。

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
