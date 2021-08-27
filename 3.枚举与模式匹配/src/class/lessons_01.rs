enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    // 分别使用 IpAddrKind 中的两个变体来创建实例
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // 可以使用任意一个变体来调用这个函数
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_type: IpAddrKind) {}
