# Rust Test

## Unit Test
  - can test private 
```
#[cfg(test)]
mod tests {
  #[test]
  fn it_works(){
    assert_eq!(4, 2 + 2)
  }

}

cargo test
```


## Integration Test
  - tests目录 与src 平级
  - 从外部调用
  - tests 目录会被特殊处理， 目录下每个测试文件都是一个单独的crate

```
  use <lib>

  #[test]
  


  carto test <function>
  carto test --test <file>
```