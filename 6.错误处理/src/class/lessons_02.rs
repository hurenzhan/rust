use std::fs::File;
use std::io::ErrorKind; // 用于描述 io 操作所可能导致的不同错误。

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn main() {
    // 可恢复错误与Result
    demo_1();

    // 匹配不同的错误
    demo_2();

    // 非 match 写法
    demo_3();

    // 失败时触发 panic 的快捷方式：unwrap 和 expect
    demo_4();
}

fn demo_1() {
    let f = File::open("hello.txt"); // File::open 函数的返回类型是 Result<T, E>
    let f = match f {
        Ok(file) => file,
        Err(e) => panic!("There was a problem opening the file: {:?}", e), // panic! 宏的输出 file: Os { code: 2, kind: NotFound, message: "系统找不到指定的文件。" }
    };

    println!("log-1 {:?}", f)
}

fn demo_2() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            // io::Error 拥有一个被称作 kind 的方法，可以通过调用它来获得 io::ErrorKin 值
            ErrorKind::NotFound => match File::create("hello.txt") {
                // 由于 File::create 本身也有可能会运行失败，所以也需要对它的返回值添加一个 match 表达式。
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };

    println!("log-2 {:?}", f)
}

fn demo_3() {
    let f = File::open("hello.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });

    println!("log-3 {:?}", f)
}

fn demo_4() {
    // unwrap 在当 Result 的返回值：
    // * Ok 变体时 unwrap 就会返回 Ok 内部的值。
    // * Err 变体时，unwrap则会替我们调用 panic! 宏。
    let f1 = File::open("hello.txt").unwrap();

    // expect 允许我们在 unwrap 的基础上指定 panic! 所附带的错误提示信息。
    let f2 = File::open("hello.txt").expect("???");

    println!("log-4 {:?}", f1);
    println!("log-4 {:?}", f2);
}
