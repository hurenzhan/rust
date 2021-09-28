// 不可恢复错误与 panic!

fn main() {
    // panic!("crash and burn");

    // 使用 panic! 产生的回溯信息
    let v = vec![1, 2, 3];
    v[99];
}
