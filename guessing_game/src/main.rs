use rand::Rng; // 定义了随机数生成器需要实现的方法集合
use std::cmp::Ordering;
use std::io; // 与 Result 相同，Ordering 也是一个枚举类型，它拥有 Less、Greater 及 Equal 这3个变体

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    loop {
        // 1. rand::thread_rng 会返回一个特定的随机数生成器
        // 2. gen_range 生成随机数的方法
        let secret_number = rand::thread_rng().gen_range(1, 5); // 含下限但不包含上限

        // 1. 函数 String::new 后返回的结果：一个新的String实例
        // 2. new函数在这里会创建一个新的空白字符串
        let mut guess = String::new(); // 存储用户输入数据，声明一个字符串类型变量

        // 1. 调用io 标准输入 stdin，会返回 std::io::Stdin 的一个实例
        // 2. 调用标准输入句柄 read_line 来获得用户输入
        // 3. & 表示参数是一个引用。通过引用在不同的地方访问同一份数据，而无须付出多余的拷贝开销。
        // 4. io::Result(read_line) 返回一个 io::Result 值，是一个枚举类型
        // 5. 实例的值是 Err，那么 expect 方法就会中断当前的程序，并将传入的字符串参数显示出来。否则就返回用户输入结果
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 1. Result包含了Ok与Err两个变体，用match表达式匹配
        // 2. continue会使程序直接跳转至下一次循环
        let guess: u32 =  match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; // 转 number 类型
        println!("You guessed: {}", guess);
        println!("The secret number is: {}", secret_number);

        // 1. cmp 方法能够为任何可比较的值类型计算出它们比较后的结果，基于该返回值的具体内容使用 match 表达式来决定下一步执行的代码
        // 2. Rust中的 match 结构及模式是工具，它们提供了依据不同条件执行不同代码的能力
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
