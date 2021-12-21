#[cfg(test)]
mod tests {
    #[test] // 它将当前的函数标记为一 =个测试，并使该函数可以在测试运行过程中被识别出来。
    fn it_works() {
        assert_eq!(2 + 2, 4);   // assert_eq! 宏断言 2+2 和 4 相等
    }
}
