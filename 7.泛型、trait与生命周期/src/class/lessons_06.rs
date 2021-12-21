// 结构体定义中的生命周期标注

use std::fmt::Display;

fn main() {
    demo_1();
    demo_2();
}

fn demo_1() {
    // 结构体中持有了引用（first_sentence），所以它的定义中需要添加生命周期标注
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        // 这个标注意味着 ImportantExcerpt 实例的存活时间不能超过存储在 part 字段中的引用的存活时间。
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("log-1 {:?}", i);
}

fn demo_2() {
    // 额外的 ann 参数，这个参数的类型为泛型 T。根据 where 从句中的约束，该参数的类型可以被替换为任何实现了 Display trait 的类型
    // 这个额外的参数会在函数比较字符串切片长度之前被打印出来，所以需要 Display 来作为 trait 约束。因为生命周期也是泛型的一种，所以生命周期参数'a和泛型参数T都被放置到了函数名后的尖括号列表中。
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
