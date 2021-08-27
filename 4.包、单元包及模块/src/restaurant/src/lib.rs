// crate
//  └── front_of_house
//  ├── hosting
//  │ ├── add_to_waitlist
//  │ └── seat_at_table
//  └── serving
//  ├── take_order
//  ├── serve_order
//  └── take_payment

mod front_of_house {
    // 整个模块树都被放置在一个名为crate的隐式根模块下。
    pub mod hosting {
        // Rust中的所有条目（函数、方法、结构体、枚举、模块及常量）默认都是私有的，所以要使用pub关键字来标记
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// 使用 use 引入
use self::front_of_house::hosting::add_to_waitlist;
// pub use crate::front_of_house::hosting::add_to_waitlist; // 通过 pub use 使一个名称可以在新作用域中被其他任意代码使用，包括外部（添加pub）

pub fn eat_at_restaurant() {
    // // 绝对路径
    // crate::front_of_house::hosting::add_to_waitlist();
    // // 相对路径
    // front_of_house::hosting::add_to_waitlist();
    // _______________________________________________
    //                      ↓
    add_to_waitlist();

    // -------------------------------------------------------------------

    // 选择黑麦面包作为夏季早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 修改我们想要的面包类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // 接下来的这一行无法通过编译，我们不能看到或更换随着食物附带的季节性水果
    // meal.seasonal_fruit = String::from("blueberries");

    // -----------------------------------------------------------------------

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// ---------------------------------------------------------------------------

// 将结构体或枚举声明为公共的
fn serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,      // 公开字段
        seasonal_fruit: String, // 默认私有
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // 用 super 关键字来跳转至 back_of_house 的父模块，也就是根模块处。找到 serve_order
    }
    fn cook_order() {}

    // ---------------------------------------------

    pub enum Appetizer {
        Soup,
        Salad,
    }
}
