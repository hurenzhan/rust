// 为 NewsArticle 与 Tweet 类型实现 Summary trait
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)") // 可以定义默认行为
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 在 impl 关键字后提供我们想要实现的 trait 名，并紧接 for 关键字及当前的类型名。
impl Summary for NewsArticle {} // 不重现默认执行默认行为

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        // 重写方法
        format!("{}: {}", self.username, self.content)
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

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best
    hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());
}
