// 函数定义声明周期标注

fn main() {
    demo_2();
}

fn demo_1() {
    // 我们将 r 的生命周期设为 a，x 的生命周期设为 b。
    // 内部的 b 代码块要小于外部的 a 生命周期代码块。
    // 在编译过程中，Rust 会比较两段生命周期的大小，并发现 r 拥有生命周期 a，但却指向了拥有生命周期 b 的内存。
    // 这段程序会由于 b 比 a 短而被拒绝通过编译：被引用对象的存在范围短于引用者。
    // ____________a________
    let r;
    {
        // ____________b________
        let x = 5;
        r = &x;
    } // ____________b________
    println!("r: {}", r);
    // ____________a________
}

// 在编译过程中会触发涉及生命周期的错误：
// 需要给返回类型标注一个泛型生命周期参数，因为 Rust 并不能确定返回的引用会指向 x 还是指向 y。
// 定义这个函数的时候，并不知道会被传入函数的具体值
// 所以也不能确定到底是 if 分支还是 else 分支会得到执行
// 也无法知晓传入的引用的具体生命周期
// 不知道 x 与 y 的生命周期是如何与返回值的生命周期相关联的。
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// ___________________________________________________
//                      ↓
// 生命周期的标注：（'）开头
// 参数与返回值中的所有引用都必须拥有相同的生命周期。
// 将这个生命周期命名为'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn demo_2() {
    // string1 直到外部作用域结束都会是有效的
    // 而 string2 的有效性则只持续到内部作用域结束的地方
    // 它可以正常地通过借用检查器进行编译
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

// 无法通过编译
fn demo_3() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    
    // 要使 println! 语句中的 result 有效，string2 需要一直保持有效，直到外部作用域结束的地方。
    // 因为我们在函数参数与返回值中使用了同样的生命周期参数
    println!("The longest string is {}", result);
}
