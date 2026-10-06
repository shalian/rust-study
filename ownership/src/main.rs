// 所有权
// 所有权规则
// 1. Rust 中的每一个值都有一个被称为其 所有者（owner）的变量
// 2. 值在任一时刻有且只有一个所有者。
// 3. 当所有者（变量）离开作用域，这个值将被丢弃。

// 变量与作用域


// String 类型
// 1. String 类型是可变的，可以存储多个字符。
// 2. String 类型的值是堆分配的，而不是栈分配的。
// 3. 当 String 类型的值离开作用域时，它会自动释放堆上的内存（自动调用 drop 函数）。

// String 类型的底层结构由三部分组成： 一个指向存放字符串内容内存的指针，一个长度（使用了多少字节的内存），和一个容量（获取了多少字节的内存）。


fn main() {
    {
        // s 在这里无效, 它尚未声明
        let s = "hello"; // 从此处起，s 开始有效

        // 使用 s
    }                    // 此作用域已结束，s 不再有效


    let mut s = String::from("hello");  // 请求其所需的内存
    s.push_str(", world!"); // push_str() 在字符串后追加字面值
    println!("{}", s); // 将打印 `hello, world!`


    let x = 5;
    let y = x; // x 的值被复制到 y
    println!("x = {}, y = {}", x, y); // 将打印 `x = 5, y = 5`


    // 变量与数据交互的方式（一）：移动
    // 当 s2 和 s1 离开作用域，他们都会尝试释放相同的内存。这是一个叫做 二次释放（double free）的错误
    let s1 = String::from("hello");
    let s2 = s1; // s1 被移动到 s2，s1 不再有效，后续使用 s1 都会出错
    println!("{}", s2); // 将打印 `hello`


    // 变量与数据交互的方式（二）：克隆
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2); // 将打印 `s1 = hello, s2 = hello`



    let s3 = String::from("hello");  // s3 进入作用域

    takes_ownership(s3);             // s3 的值移动到函数里 ...
                                     // ... 所以到这里不再有效

    let x1 = 5;                      // x1 进入作用域

    makes_copy(x1);                  // x1 应该移动函数里，
                                   // 但 i32 是 Copy 的，所以在后面可继续使用 x1


    

    let s4 = gives_ownership();         // gives_ownership 将返回值移给 s4

    let s5 = String::from("hello");     // s5 进入作用域

    let s6 = takes_and_gives_back(s5);  // s5 被移动到 takes_and_gives_back 中,它也将返回值移给 s6



    let s7 = String::from("hello");
    let (s8, len) = calculate_length(s7);
    println!("s8 = {}, len = {}", s8, len);




}


fn takes_ownership(some_string: String) { // some_string 进入作用域
  println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
  println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作



fn gives_ownership() -> String {           // gives_ownership 将返回值移动给调用它的函数

  let some_string = String::from("yours"); // some_string 进入作用域

  some_string                              // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

  a_string  // 返回 a_string 并移出给调用的函数
}



fn calculate_length(s: String) -> (String, usize) { // s 进入作用域
  let len = s.len(); // len() 返回 s 的长度

  (s, len)
}