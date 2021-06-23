默
认情况下，rust 采用多线程并行执行所有测试，当有串行需要时可以执行：cargo test -- --test-threads={thread_numbers}来控制执行测试的线程数。

rust 默认不打印 passed test 的任何输出，当有需要打印输出时，执行：cargo test -- --show-output

当期望只运行某个特定测试时，执行：cargo test {test_function_name}

当期望只运行某一类测试时，执行：cargo test {test_function_name_matcher}

期望 ignore 测试时，在测试函数上添加： #[ignore] attribute

