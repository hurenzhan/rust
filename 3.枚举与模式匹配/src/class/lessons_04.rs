// 控制流运算符match
// match，它允许将一个值与一系列的模式相比较，并根据匹配的模式执行相应代码

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let alaska = Coin::Quarter(UsState::Alaska);
    println!("log-2 {}", value_in_cents(alaska));

    // -------------------------------------------

    // 匹配Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("log-3 {:?}，{:?}", six, none);

    // -------------------------------------------

    // _ 通配符
    // let some_u8_value = 0u8;
    // match some_u8_value {
    //     1 => println!("one"),
    //     3 => println!("three"),
    //     5 => println!("five"),
    //     7 => println!("seven"),
    //     _ => println!("log-4"), // 式_来替代其余的值
    // }

    // -------------------------------------------

    // 控制流if let
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("log-5");
    }

    // -------------------------------------------
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("log-6 {}", count);
}

fn value_in_cents(coin: Coin) -> u32 {
    // 一个枚举以及一个以枚举变体作为模式的 match 表达式
    // 与if不同，这里的表达式则可以返回任何类型
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // 加入 state 的变量用于匹配变体Coin::Quarter中的值。
        // 当匹配到Coin::Quarter时，变量 state 就会被绑定到25美分所包含的值上，就可以在这个分支中使用 state 了
        Coin::Quarter(state) => {
            println!("log-1 State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
