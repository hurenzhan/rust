// use std::cmp::Ordering;
// use std::io;
//__________________________    使用嵌套的路径来清理众多use语句
//          ↓
// use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
//__________________________
//          ↓
// use std::io::{self, Write};

// use std::collections::*; // * 可以将所有公共条目都导入当前作用域

// use std::fmt::Result;
// use std::io::Result as IoResult; // 使用 as 起别名

fn main() {
    println!("hello")
}
