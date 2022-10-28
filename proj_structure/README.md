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

