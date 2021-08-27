// 可以在枚举的变体中嵌入任意类型的数据，无论是字符串、数值，还是结构体，甚至可以嵌入另外一个枚举
// struct Ipv4Addr {
//     // --略
// }
// struct Ipv6Addr {
//     // --略
// }
// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

#[derive(Debug)]
enum Message {
    Quit,                       // 没有任何关联数据
    Move { x: i32, y: i32 },    // 包含了一个匿名结构体
    Write(String),              // 包含了一个String
    ChangeColor(i32, i32, i32), // 包含了3个i32值
}

// 可以使用 impl 关键字定义结构体的方法一样，同样可以定义枚举的方法
impl Message {
    fn call(&self) {
        println!("{:?}", self)
    }
}

fn main() {
    let m = Message::Quit(String::from("hello"));
    m.call();
}
