mod front_of_house; // 在 mod front_of_house 后使用分号而不是代码块，会前往与当前模块同名的文件中加载模块内容。
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
