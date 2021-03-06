# 概念
1. 使用包含特定规则的所有权系统来管理内存，这套规则允许编译器在编译过程中执行检查工作，而不会产生任何的运行时开销。
2. 掌握了所有权及其相关工具，就可以将这些问题交给 Rust 处理，减轻用于思考栈和堆的心智负担。

# 栈与堆
1. 所有存储在栈中的数据都必须拥有一个已知且固定的大小。
2. 对于那些在编译期无法确定大小的数据，你就只能将它们存储在堆中
3. 希望将数据放入堆中时，可以请求特定大小的空间。操作系统会根据你的请求在堆中找到一块足够大的可用空间，将它标记为已使用，并把指向这片空间地址的指针返回。这一过程就是堆分配，它也常常被简称为分配。
4. 将值压入栈中不叫分配。由于指针的大小是固定的且可以在编译期确定，所以可以将指针存储在栈中。当你想要访问指针所指向的具体数据时，可以通过指针指向的地址来访问。

# 所有权规则
1. Rust 中的每一个值都有一个对应的变量作为它的所有者。
2. 在同一时间内，值有且仅有一个所有者。
3. 当所有者离开自己的作用域时，它持有的值就会被释放掉。

# 总结
1. 所有权、借用和切片的概念是 Rust 可以在编译时保证内存安全的关键所在。
2. 像其他系统级语言一样，Rust语言给予了程序员完善的内存使用控制能力。还能够自动清除那些所有者离开了作用域的数据。
3. 所有权影响了 Rust 中绝大部分功能的运作机制。
