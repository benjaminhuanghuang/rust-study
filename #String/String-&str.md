String is `UTF-8` coded, `可变长`的字符串. String 对字符串的内容有 `ownership`

```
  let s  = String.from("Hello");
```

String 实现了

```
  <code>[Deref]<Target = [str]><code>
```

因此继承了 str 的所有方法,可以通过&操作变成&str

String 由 3 部分组成: pointer, length, capacity
