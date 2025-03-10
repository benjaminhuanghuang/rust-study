# 异步编程 vs. 多线程

在高并发场景下，异步编程和多线程（多进程）都是常见的并发处理方式，但 异步编程（async/await）相比多线程编程（threads）在许多情况下更高效，主要体现在 性能、资源占用、任务调度 等方面。

## 多线程 (Threads) 的特点

多线程编程是通过 创建多个线程 来并行执行多个任务，适用于 CPU 密集型任务，如：

并行计算（矩阵运算、图像处理）
多核心 CPU 任务（科学计算、大数据分析）
优点
✅ 利用多核 CPU 资源：多个线程可以真正并行执行计算任务。
✅ 易于理解：编写代码时，每个线程像独立的任务。

缺点
❌ 线程切换（上下文切换）成本高：

CPU 在多个线程之间切换时，会保存和恢复寄存器、堆栈，消耗资源。
在高并发（如 10K+ 线程）时，线程切换会成为性能瓶颈。
❌ 高内存占用：

每个线程有自己的 堆栈（Stack），通常 几 MB，如果有 1 万个线程，会占用 GB 级别 内存。
❌ 死锁 & 竞态条件：

多个线程访问同一资源时需要 锁（Mutex），容易出现 死锁、数据竞争 问题。

## 异步编程 (Async/Await) 的特点

异步编程 使用 事件驱动（Event Loop）+ 非阻塞 I/O 来高效处理任务，适用于 高并发 & I/O 密集型任务：

网络请求（Web 服务器、API 网关）
文件 I/O（日志、数据库读写）
定时任务（调度任务、后台任务）
优点
✅ 低线程数，支持高并发：

单线程可处理数万个任务，因为异步任务不会阻塞线程，而是 等待 I/O 完成后继续执行。
适合 Web 服务器（如 Rust tokio、Node.js）。
✅ 低内存占用：

异步任务不会创建新线程，它们在同一个线程上 协作运行，只需要少量堆栈空间（KB 级别）。
✅ 避免线程同步问题：

不需要锁（Mutex），减少了 死锁、数据竞争 的风险。
缺点
❌ 不适合 CPU 密集型任务：

如果任务需要 大量计算（如视频处理、AI 计算），异步无法提升 CPU 计算效率，仍然需要多线程或多进程。
❌ 代码结构复杂：

需要 async/await 语法，不能直接使用 for/while 等同步代码逻辑。
