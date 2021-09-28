fn main() {
    let s = String::from("hello"); // 变量 s 进入作用城
                                   // takes_ownership(s);  // s 的值被移动进了函数
                                   // 所以 s 从这里并始不再有效

    // 1. let s = takes_ownership(s); // 通过返回值转移所有权
    // --1 println!("log-s {}", s);

    takes_ownership(&s); // 通过引用符
    println!("log-s {}", s);

    let x = 5; // 变量 x 进入作用域
    makes_copy(x);
    // 由于 i32 是 Copy 的,所以我们像然可以在这之后使用 x
}

// 通过返回值转移所有权
// fn takes_ownership(value: String) -> (String, usize) {
//     println!("log-1 {}", value);
//     (value)
// } // 在这里离开作用于, drop函数被自动调用，s所占用的内存也就随之被释放了

// 通过引用符表示引用值
fn takes_ownership(value: &String) {
    println!("log-1 {}", value);

    // 引用是默认不可变的，需要添加mut标识符
    // value.push_str(", world"); // error[E0596]: cannot borrow immutable borrowed content

    // --1 (value)
} // 在这里离开作用于, drop函数被自动调用，s所占用的内存也就随之被释放了

fn makes_copy(value: i32) {
    println!("log-2 {}", value);
} // 离开作用于，没什么特别的事情发生

// -----------------------------------------------------
// 一个作用域一次只能声明一个可变引用
// let mut s = String::from("hello");
// let r1 = &mut s;
// let r2 = &mut s;
// error[E0499]: cannot borrow `s` as mutable more than once at a time
// 这条限制性规则可以帮助我们在编译时避免数据竞争。
// 它会在指令满足以下3种情形时发生：
// • 两个或两个以上的指针同时访问同一空间。
// • 其中至少有一个指针会向空间中写入数据。
// • 没有同步数据访问的机制。
// 数据竞争会导致未定义的行为，由于这些未定义的行为往往难以在运行时进行跟踪。
// 也就使得出现的bug更加难以被诊断和修复。
// 这规则避免了这种情形的出现，因为存在数据竞争的代码连编译检查都无法通过！

// 可以通过花括号来创建一个新的作用域范围来创建多个可变引用
// let mut s = String::from("hello");
// {
//  let r1 = &mut s;
// } // 由于 r1 在这里离开了作用域，所以我们可以合法地再创建一个可变引用
// let r2 = &mut s;

// 不能在拥有不可变引用的同时创建可变引用。
// 同时存在多个不可变引用是合理合法的，对数据的只读操作不会影响到其他读取数据的用户。
// -----------------------------------------------------