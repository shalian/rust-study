fn main() {
    println!("Hello, world!");

    anathor_function();
    anathor_function_with_argument(10);


    let y = {
        let x = 3;
        x + 1  // 表达式的结尾没有分号。如果在表达式的末尾加上分号，那么它就转换为语句，而语句不会返回值。
    };

    println!("The value of y is: {}", y);


    let five = five();
    println!("five: {}", five);
}


fn anathor_function() {
    println!("anathor_function");
}

fn anathor_function_with_argument(x: i32) {
    println!("anathor_function_with_argument: {}", x);
}

// 带有返回值的函数
fn five() -> i32 {
    5
}