## Race conditions
假设有一个共享变量 balance, 进程 A 的操作为
```
balance = balance - 100
```
进程 B 的操作为
```
balance = balance - 200
```
这些编程语句最终都将转化为多条指令的执行, 比如减 100 的指令为
```
A1. LOAD R1, BALANCE
A2. SUB R1, 100
A3. STORE BALANCE, R1
```
那么类似的, 减 200 的指令为
```
B1. LOAD R1, BALANCE
B2. SUB R1, 200
B3. STORE BALANCE, R1
```

cpu 在执行指令时, 某一时刻具体执行那条指令并不是确定的, 会有上下文切换发生, 上面六条指令的执行可能会是
```
A1. LOAD R1, BALANCE
A2. SUB R1, 100
A3. STORE BALANCE, R1
    Context Switch!
    B1. LOAD R1, BALANCE
    B2. SUB R1, 200
    B3. STORE BALANCE, R1
```
balance 最终被减去了 300. 但是也有可能是这样的
```
A1. LOAD R1, BALANCE
A2. SUB R1, 100
    Context Switch!
    B1. LOAD R1, BALANCE
    B2. SUB R1, 200
    B3. STORE BALANCE, R1
Context Switch!
A3. STORE BALANCE, R1
```
这里 balance 实际上只减去了 200.

## Deadlock


