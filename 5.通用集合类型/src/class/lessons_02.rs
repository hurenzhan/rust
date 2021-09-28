// 使用字符串存储UTF-8编码的文本

fn main() {
    // 创建
    create();

    // 更新
    update();

    // 索引/切片
    index_of();

    // 遍历
    for_each();
}

fn create() {
    // 可以从new函数开始来创建一个字符串
    // let mut s = String::new();;
    // _______________________________________
    //              ↓
    let data = "initial contents";
    let s = data.to_string(); // 使用 to_string 方法基于字符串字面量创建 String，与 String::from("initial contents") 相同
    println!("log-1 {}", s);
}

fn update() {
    let mut a1 = String::from("foo");
    let a2 = "bar";
    a1.push_str(a2);
    println!("log-2 {}", a2);

    // _______________________________________________

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // 这里的 + 运算符会调用一个add方法，所以 s2 采用了引用 -> fn add(self, s: &str) -> String {}
    // * 我们只能将 String 与 &str 相加，而不能将两个String相加。
    // * 编译器可以自动将 &String 类型的参数强制转换为 &str 类型。
    // * 当我们调用 add 函数时，Rust使用了一种被称作解引用强制转换的技术，将&s2转换为了&s2[..]。
    // * add 函数签名中的 self 并没有 & 标记，所以 add 函数会取得 self 的所有权。这也意味着 s1 将会被移动至 add 函数调用中，并在调用后失效。
    // * 实际上这条语句会取得 s1 的所有权，再将 s2 中的内容复制到其中，最后再将 s1 的所有权作为结果返回。
    let s3 = s1 + &s2; // 注意这里的s1已经被移动且再也不能被使用了
    println!("log-3 {}", s3);

    // _______________________________________________

    // format! 宏
    let b1 = String::from("tic");
    let b2 = String::from("tac");
    let b3 = String::from("toe");
    // format! 宏与 println! 宏的工作原理完全相同
    // format! 会将结果包含在一个 String 中返回，并且不会夺取任何参数的所有权。
    let s = format!("{}-{}-{}", b1, b2, b3);
    println!("log-4 {}", s);
}

fn index_of() {
    // Rust 中的字符串并不支持索引。
    // * String 实际上是一个基于 Vec<u8> 的封装类型。
    // * 用 UTF-8 编码来存储字符串所需要的字节数（字节、标量值及字形簇）。 例如 "Здравствуйте" 每个 Unicode 标量值都需要占据 2 字节
    // * 索引操作的复杂度往往会被预期为常数时间（O (1)）。但在 String 中无法保障这种做法的性能，因为要遍历从头至索引位置的整个内容，来确定究竟有多少合法的字符存在。
    // 所以尝试通过索引引用字符串通常是一个坏主意，因为字符串索引操作应当返回的类型是不明确的：究竟应该是字节，还是字符，或是字形簇，甚至是字符串切片呢？

    // _______________________________________________

    // 字符串切片
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("log-5 {}", s); // Зд 俄文边界是 2 字节编码一个字符串
}

fn for_each() {
    println!("log-6");
    // chars()：会返回所有类型为 char 的值
    // bytes()：会返回所有原始字节
    for c in "Здравствуйте".chars() {
        println!("{}", c);
    }

    for c in "Здравствуйте".bytes() {
        println!("{}", c);
    }
}
