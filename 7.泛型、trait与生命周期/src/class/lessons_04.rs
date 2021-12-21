// 为 NewsArticle 与 Tweet 类型实现 Summary trait
pub trait Summary {
    fn summarize_author(&self) -> String;

    // 还可以在默认实现中调用相同 trait 中的其他方法
    // trait 可以在只需要实现一小部分方法的前提下，提供许多有用的功能。
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // 重写方法
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // 使用 trait 作为参数
    notify(tweet);

    // 返回实现了 trait 的类型
    let rs = returns_summarizable();

}

// 参数 item 可以是任何实现了 Summary trait 的类型。使用 impl Trait 语法糖：适用于短小的示例
// pub fn notify<T: Summary>(item: T) // 完整形式：适用于复杂情形 pub fn notify<T: Summary>(item1: T, item2: T)
// _______________________________________________
//                ↓
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// __________________________________________________________________

// 通过 + 语法来指定多个 trait 约束
// 假如 notify 函数需要在调用 summarize 的同时显示格式化后的 item，那么 item 就必须实现 trait：Summary 和 Display。
// pub fn notify(item: impl Summary + Display) {}
// pub fn notify<T: Summary + Display>(item: T) {}

// _____________________________________________________________________

// 使用 where 从句来简化 trait 约束
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}
// _______________________________________________
//                ↓
// fn some_function<T, U>(t: T, u: U) -> i32 where T: Display + Clone, U: Clone + Debug {}

// _____________________________________________________________________

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
