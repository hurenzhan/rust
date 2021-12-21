use std::cmp::PartialOrd;
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

// 在函数定义中泛型数据类型
// PartialOrd：要使用（>）运算符来比较两个 T 类型的值。由于这一运算符被定义为标准库 trait std::cmp::PartialOrd 的一个默认方法，所以需要在 T 的 trait 约束中指定 PartialOrd，才能够使 largest 函数用于任何可比较类型的切片上。
// + Copy：无法从不可复制的切片[T]中移出元素，i32 或 char 已经实现了Copy trait。但将 largest 函数泛型化时，list 参数中的类型有可能是没有实现 Copy trait，所以把 Copy 加入 T 的 trait 约束中
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
