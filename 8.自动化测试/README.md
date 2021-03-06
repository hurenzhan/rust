# 概念
1. 程序中的正确性被用来衡量一段代码的实际行为与设计目标之间的一致程度。
2. Rust 将程序正确性作为一项非常优先的考量因素，但是一个程序最终是否正确，终究是复杂并且难以证明的。
3. Rust的类型系统为我们提供了相当多的安全保障，但还是不足以防止所有的错误。

# 语言层面
1. 内置了编写测试代码、执行自动化测试任务的功能。
2. 给以使用断言让 Rust 知道我们的意图去测试。

# 规范
1. 准备所需的数据或状态。
2. 调用需要测试的代码。
3. 断言运行结果与我们所期望的一致。

# demo
1. cargo new adder --lib 新建一个名为 adder 的库项目。
2. adder 库会自动生成一个 src/lib.rs 文件。
3. cargo test 会运行项目中的所有测试。

# 总结
1. 代码一旦发生 panic，就再也不能恢复了。只要决定某种情形是不可恢复的，那么就可以使用 panic! ，而不用考虑错误是否存在可以恢复的机会。
2. 当选择返回一个 Result 值时，就将这种选择权交给了调用者。调用者可以来决定是否要尝试进行恢复。
3. 或者当返回 Result 时干脆认为 Err 是不可恢复的，并使用 panic! 变为不可恢复错误。因此，我们会在定义一个可能失败的函数时优先考虑使用 Result 方案。

