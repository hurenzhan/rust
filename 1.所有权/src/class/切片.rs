fn main() {
    // let my_string = String::from("hello world");
    // let word = first_word(&my_string[..]); // // first_word 可以接收 String 对象的切片作为参数
    let my_string_literal = "hello world";  // 类型其实就是 &str：它是一个指向二进制程序特定位置的切片。正是由于 &str 是一个不可变的引用，所以字符串字面量自然才是不可变的
    // let word = first_word(&my_string_literal[..]);
    let word = first_word(my_string_literal);
    println!("the first word is : {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// 创建一个 String 的切片引用，而不是对整个字符串本身的引用。
// let s = String::from("hello world");
// let hello = &s[0..5];
// let world = &s[6..11];
// let all = &s[..];

// let a = [1, 2, 3, 4, 5];
// let a = [1, 2, 3, 4, 5];
// let slice = &a[1..3];
