// 使用结构体更新语法，根据其他实例创建新实例

fn main() {
    // 一旦实例可变，那么实例中的所有字段都将是可变的。
    let mut user1 = build_user(String::from("someone@example.com"), String::from("huren"));
    user1.email = String::from("anotheremail@example.com");
    println!(
        "{}, {}, {}, {}",
        user1.username, user1.sign_in_count, user1.active, user1.email
    );

    // 这里的双点号..表明剩下的那些还未被显式赋值的字段都与给定实例拥有相同的值。
    let user2 = User {
        email: String::from("user2@example.com"),
        username: String::from("user2"),
        ..user1
    };
    println!(
      "{}, {}, {}, {}",
      user2.username, user2.sign_in_count, user2.active, user2.email
  );
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // 由于字段 email 与参数拥有相同的名字，只保留 email 即可。
        username,
        active: true,
        sign_in_count: 1,
    }
}
