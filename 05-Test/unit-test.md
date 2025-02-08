# Unit Test

Unit Test 都会被放到一个名为 tests, 带有 #[cfg(test)] 属性 的模块中，测试函数要加上 #[test] 属性。

The cfg enables conditional compilation, so this module will be compiled only when testing.

注意私有的函数也可以被测试！

期望 ignore 测试时，在测试函数上添加： #[ignore] attribute

```
#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        // 这个断言会导致测试失败。注意私有的函数也可以被测试！
        assert_eq!(bad_add(1, 2), 3);
    }
}
```

## Put test in a seperate folder

```
// region:    Test
#[cfg(test)]
#[path = "../_tests/model_todo.rs"]
mod tests;
// endregion: Test
```
