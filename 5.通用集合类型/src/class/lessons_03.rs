use std::collections::HashMap;

// 在哈希映射中存储键值对 HashMap<K,V>
// 为了提供抵御拒绝服务攻击（DoS，Denial of Service）的能力，HashMap 默认使用了一个在密码学上安全的哈希函数。

fn main() {
    // 创建
    create();

    // 所有权
    ownership();

    // 访问哈希映射中的值
    get_value();

    // 更新
    update_value();
}

fn create() {
    let mut scores1 = HashMap::new();
    scores1.insert(String::from("Blue"), 10);
    scores1.insert(String::from("Yellow"), 50);
    println!("log-1 {:?}", scores1);

    // _______________________________________________

    // collect 方法可以将数据收集到很多数据结构中，这些数据结构也包括 HashMap
    // 假设我们在两个不同的动态数组里分别存储了队伍的名字和分数，那么我们就可以使用 zip 方法来创建一个元组的数组。
    // 接着还可以使用 collect 方法来将动态数组转换为哈希映射
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // iter()：转化为元祖数组
    // zip：创建元祖数组，规则位每个数组第一个拼成一个元祖
    // collect：将动态数组转换为哈希映射
    // 指明的 HashMap<_, _> 不能被省略，因为 collect 可以作用于许多不同的数据结构
    // _占位，Rust能够根据动态数组中的数据类型来推导出哈希映射所包含的类型。
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("log-2 {:#?}", scores2);
}

fn ownership() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // 其值将会转移且所有权会转移给哈希映射

    //println!("log-3 {}, {}", field_name, field_value)   // 刻开始失效，若尝试使用它们则会导致编译错误！
}

fn get_value() {
    // get 方法来获得哈希映射中的值

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("log-4 {:?}", score); // 返回的 Some(&10)。因为 get 返回的是一个 Option<&V>

    // ______________________________________________________

    // for循环来遍历哈希映射中所有的键值对
    for (key, value) in &scores {
        println!("log-5 {}: {}", key, value);
    }
}

fn update_value() {
    let mut scores1 = HashMap::new();
    scores1.insert(String::from("Blue"), 10);
    scores1.insert(String::from("Blue"), 25);
    println!("log-6 {:?}", scores1);

    // ___________________________________________________________

    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);

    // entry 接收我们想要检测的键作为参数
    // Entry 的 or_insert 方法被定义为返回一个 Entry 键所指向值的可变引用，假如这个值不存在，就将参数作为新值插入哈希映射中，并把这个新值的可变引用返回。
    scores2.entry(String::from("Yellow")).or_insert(50);
    scores2.entry(String::from("Blue")).or_insert(50);

    println!("log-7 {:?}", scores2); // {"Blue": 10, "Yellow": 50} 键存在就不更新

    // _______________________________________________

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; // 新值的可变引用通过 or_insert 返回
    }

    println!("log-7 {:?}", map);
}