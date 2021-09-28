// 不需要对字段命名的元组结构体，来创建不同的类型

// 这里的 black 和 origin 是不同的类型，因为它们两个分别是不同元组结构体的实例。
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

#[derive(Debug)] // 必须为自己的结构体显式地选择这一功能。在结构体定义前添加了 #[derive(Debug)] 注解
struct Rectangle {
    width: u32,
    height: u32,
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
// ______________________________________
//                  ↓
// 1. 方法与函数十分相似：它们都使用 fn 关键字及一个名称来进行声明；
// 2. 也可以拥有参数和返回值；
// 3. 方法与函数依然是两个不同的概念，方法总是被定义在某个结构体（或者枚举类型、trait对象）的上下文中，第一个参数是 self，用于指代结构体实例。
// 4. 可以拥有多个 impl 块
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数
    // 可以定义不用接收self作为参数的函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // :? 或 :#? 会告知 println! 当前的结构体需要使用名为 Debug 的格式化输出。
    // Debug 是另外一种格式化 trait，它可以让我们在调试代码时以一种对开发者友好的形式打印出结构体
    println!("rect1 is {:?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    // ----------------------------------------------------------------------------------------

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // ----------------------------------------------------------------------------------------

    let sq = Rectangle::square(40);
    println!("sq {:#?}", sq);
}
