fn main() {
    // 由于支量s还未被声明,所以它在这里是不可用的
    let s = "hello"; // 从这里开始变量s变得可用
    println!("log-1 {}", s);

    string_type(); // string类型
    data_clone(); // 数据克隆
} // 作用越到这里结束,变量s再次不可用

// 1. Rust在变量离开作用域时，会调用一个叫作drop的特殊函数
// 2. String类型的作者可以在这个函数中编写释放内存的代码。
// 2. Rust会在作用域结束的地方（即}处）自动调用drop函数。
fn string_type() {
    // 1. 这个类型会在堆上分配到自己需要的存储空间，所以它能够处理在编译时未知大小的文本。
    // 2. 双冒号（::）运算符允许我们调用置于String命名空间下面 的特定from函数
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() 函数向String空间的尾部添加了一段字面量
    println!("log-2 {}", s);
}

// 变量和数据交互的方式：移动
// --------------------------------

// let s1 = String::from("hello");
// let s2 = s1; // 我们可以说s1被移动到了s2中，s1变为无效变量。
// println!("{}, world!", s1);  // error[E0382]: use of moved value: `s1`
// * 当我们将s1赋值给s2时，便复制了一次String的数据，这意味着复制了它存储在栈上的指针、长度及容量字段。但没有复制指针指向的堆数据。
// * 假如Rust依照这样的模式去执行赋值，那么当堆上的数据足够大时，类似于s2 = s1这样的指令就会造成相当可观的运行时性能消耗。
// * 两个指针指向了同一个地址，这就导致当 s2 和 s1 离开自己的作用域时，它们会尝试去重复释放相同的内存。这也就是内存错误之一，二次释放。重复释放内存可能会导致某些正在使用的数据发生损坏，进而产生潜在的安全隐患。
// * Rust在这种场景下会将s1废弃，不再视其为一个有效的变量。不需要在 s1 离开作用域后清理任何东西。

// let x = 5;
// let y = x;
// println!("x = {}, y = {}", x, y);
// * x在被赋值给y后也依然有效，且没有发生移动现象
// * 因为类似于整型的类型可以在编译时确定自己的大小，能够将自己的数据完整地存储在栈中，对于这些值的复制操作是非常快速的。

// ---------------------------------

// 变量和数据交互的方式：克隆
fn data_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("log-3 s1 = {}, s2 = {}", s1, s2);
}


/*
一般来说，任何简单标量的组合类型都可以是Copy的，任何需要分配内存或某种资源的类型都不会是Copy的。
  Copy这种trait的类型：
  • 所有的整数类型，诸如u32。
  • 仅拥有两种值（true和false）的布尔类型：bool。
  • 字符类型：char。
  • 所有的浮点类型，诸如f64。
  • 如果元组包含的所有字段的类型都是Copy的，那么这个元组也 是Copy的。例如，(i32, i32)是Copy的，但(i32, String)则不是。
*/
