// 传播错误
// 当编写的函数中包含了一些可能会执行失败的调用时，除了可以在函数中处理这个错误，还可以将这个错误返回给调用者来决定应该如何做进一步处理。

use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Read};

fn main() {
    let s1 = read_username_from_file_1();
    let s2 = read_username_from_file_2();
    let s3 = read_username_from_file_3();
    let s4 = read_username_from_file_4();
    let dyn_err = dyn_err();

    println!("log-1 {:?}", s1);
    println!("log-2 {:?}", s2);
    println!("log-3 {:?}", s3);
    println!("log-4 {:?}", s4);
    println!("log-5 {:?}", dyn_err);
}

// 普通版
fn read_username_from_file_1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // 错误作为结果返回，而不是调用 panic!
    };

    let mut s = String::new(); // 接受文件内容的变量

    // 使用 match 匹配，因为 read_to_string 方法可能会失败，恰好同样使用了 io::Error 作为错误类型。
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 运算符 ? 返回 panic!
fn read_username_from_file_2() -> Result<String, io::Error> {
    // 问号运算符 ? 是简化传播错误的语法
    // 假如值是 Err，那么这个值就会作为整个程序的结果返回，如同使用了 return 一样将错误传播给调用者。
    // ? 运算符只能被用于返回 Result 的函数

    let mut f = File::open("hello.txt")?;

    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)
}

// 链式方法调用
fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// 简化版
fn read_username_from_file_4() -> Result<String, io::Error> {
    // fs::read_to_string 用于打开文件，创建一个新 String，并将文件中的内容读入这个 String
    let f = fs::read_to_string("hello.txt")?;
    Ok(f)
}

// 任何可能的错误类型
fn dyn_err() -> Result<(), Box<dyn Error>> {
    // 的 Box<dyn Error> 被称作 trait 对象，可以理解任何可能的错误类型
    File::open("hello?.txt")?;

    Ok(())
}
