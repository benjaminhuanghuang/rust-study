```
struct AlwaysEqual;

let subject = AlwaysEqual;

// 不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为Unit-like struct，然后再为它实现某个特征
impl SomeTrait for AlwaysEqual {

}
```