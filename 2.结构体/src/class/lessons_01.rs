struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

fn main() {
  // 一旦实例可变，那么实例中的所有字段都将是可变的。
  let mut user1 = User {
      email: String::from("someone@example.com"),
      username: String::from("someusername123"),
      active: true,
      sign_in_count: 1,
  };
  user1.email = String::from("anotheremail@example.com");
  println!("{}, {}, {}", user1.username, user1.sign_in_count, user1.active)
}