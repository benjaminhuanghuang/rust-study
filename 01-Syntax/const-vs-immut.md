1. module global, 常量可以在任何范围内声明，包括全局范围；
2. valued is defined at compile time, const 只能设置为固定值或常量表达式，在编译时就能计算出其结果；
3. const 使用 const 关键字声明，必须指定数据类型，而变量可以推断出其类型；
4. 的命名必须使用大写字母或数字和下划线；
```
  const READY_TO_START: i32 = 2;

  const U_X2 :u32 = 100_1000; //在数字中插入下划线可提高可读性
```

