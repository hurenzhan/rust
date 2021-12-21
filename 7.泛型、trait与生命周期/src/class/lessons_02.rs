fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// 可以为结构体或枚举实现方法，而方法也可以在自己的定义中使用泛型。
struct Point<T, U> {
    x: T,
    y: U,
}
// 必须紧跟着 impl 关键字声明 T U，以便能够在实现方法时指定类型 Point<T, U>。通过在 impl 之后将 T 声明为泛型，Rust 能够识别出 Point 尖括号内的类型是泛型而不是具体类型。
impl<T, U> Point<T, U> {    // 泛型参数 T 与 U 被声明在impl之后，因为它们是结构体定义的一部分。
    // 泛型参数 V 与 W 则被定义在 fn mixup 中，因为它们仅仅与方法本身相关。
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
