// 希望使用4个u8值来代表V4地 址，并依然使用String值来代表V6地址，那么结构体就无法轻易实现
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}
