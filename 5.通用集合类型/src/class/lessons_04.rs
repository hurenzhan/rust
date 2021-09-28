fn main() {
    // 给定一组整数，使用动态数组来计算该组整数中的：
    // * 平均数、中位数（对数组进行排序后位于中间的值）
    // * 众数（出现次数最多的值）。
    demo_1();
}

fn demo_1() {
    let mut list = vec![3, 10, 2, 5, 6, 4, 0];
    let mut average = 0;
    let list_len = &list.len();

    for i in &list {
        average += i;
    }
    average = average / list_len;

    println!("demo_1 average: {:?}", average);

    // ___________________________________________________

    list.sort_unstable();


    println!("{:?}", list);
    
}
