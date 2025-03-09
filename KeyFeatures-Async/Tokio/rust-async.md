```
async fun foo() -> usize {

}
```

turns to

```
fu foo() ->impl Future<Output=usize> {
  async {
    0
  }
}

```
