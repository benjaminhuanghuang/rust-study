## Folder structure
```
project
   |
   ——src  【项目代码目录】
   |
   ——targets 【编译后自动生成目录】
   |
   ——tests 【测试用例目录】
   |
   ——bench 【性能测试目录】
   |
   ——example 【用例目录】
   |
   ——cargo.toml 【不写[[bin]]标签默认执行src/main.rs】
```

## Package
`cargo new` will create a package

src/main.rs  is the root of the package

`cargo build` will create a bin, which has the same name as package

A package can have 1 `lib` create, and N `bin` create

## Crate



## Module
```
mod nation {
    mod government {
        fn govern() {}
    }
    mod congress {
        fn legislate() {}
    }
    mod court {
        fn judicial() {}
    }
}
```



