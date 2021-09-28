// 使用动态数组存储多个值

fn main() {
    // 创建
    new_vec();

    // 读取
    read();

    // 遍历
    for_each();

    // 枚举存储多个类型的值
    multiple();
}

fn new_vec() {
    // 创建一个用来存储i32数据的空动态数组
    // 在实际中，向动态数组内插入了数据，Rust便可以在绝大部分情形下推导出你希望存储的元素类型。
    // let v: Vec<i32> = Vec::new();
    // _______________________________________
    //              ↓
    // let v = vec![1, 2, 3];  // 宏可以根据我们提供的值来创建一个新的动态数组。

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("log-1 {:?}", v)
} // 和其他的 struct 一样，动态数组一旦离开作用域就会被立即销毁

fn read() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("log-2 {}", third);

    // get 方法来访问动态数组中的元素
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // 一旦程序获得了一个有效的引用，借用检查器就会执行所有权规则和借用规则，来保证这个引用及其他任何指向这个动态数组的引用始终有效。
    // 所有权规则：我们不能在同一个作用域中同时拥有可变引用与不可变引用。
    // let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
    // let first = &v[0]; -> let first = v[0];  ✔
    // v.push(6);
    // println!("The first element is: {}", first);
    //
    // 编译这段代码将会导致下面的错误，是由动态数组的工作原理导致的：
    // * 动态数组中的元素是连续存储的，插入新的元素后也许会没有足够多的空间将所有元素依次相邻地放下。
    // * 这就需要分配新的内存空间，并将旧的元素移动到新的空间上。
    // * 在本例中，第一个元素的引用可能会因为插入行为而指向被释放的内存。借用规则可以帮助我们规避这类问题。
}

fn for_each() {
    let mut v = vec![100, 200, 300];
    for i in &v {
        println!("log-3 {}", i)
    }

    for i in &mut v {
        // += 运算符来修改可变引用指向的值，需要使用解引用运算符（*）来获得i绑定的值。
        *i += 50;
        println!("log-4 {}", i)
    }
}

fn multiple() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    match &row[2] {
        SpreadsheetCell::Text(state) => {
            println!("State quarter from {:?}!", state);
        }
        SpreadsheetCell::Int(state) => {
            println!("State quarter from {:?}!", state);
        }
        SpreadsheetCell::Float(state) => {
            println!("State quarter from {:?}!", state);
        }
    }

    println!("log-5 {:?}", row[2]);
}
