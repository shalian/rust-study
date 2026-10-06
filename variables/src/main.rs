fn main() {
    let x = 5; // 不能重新赋值，会报错
    let mut y = 5; // 可以重新赋值
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);


    // 定义常量，不能重新赋值
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


    // 遮蔽
    // 使用 let 关键字时有效地创建了一个新的变量，而不是重新赋值旧的变量。
    // 可以设置不同类型的数据。
    // mut 设置变量可更改，但不能修改类型
    let c = 10;
    println!("The value of c is: {}", c);
    let c = c + 1;
    println!("The value of c is: {}", c);
}
