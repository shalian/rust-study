#[derive(Debug)]

enum IpAddrKind {
    V4,
    V6,
}

// 枚举替代结构体
// V4 和 V6 都关联了 String 类型
enum IpAddr {
    V4(String),
    V6(String),
}

// 每个成员可以处理不同类型和数量的数据
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}



enum Message {
    Quit,                        // 没有关联任何数据
    Move { x: i32, y: i32 },     // 包含一个匿名结构体
    Write(String),               // 包含一个 String 类型
    ChangeColor(i32, i32, i32),  // 包含三个 i32 类型
}
// 可以在枚举上定义方法。
impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}


// Option 枚举
enum Option<T> {
    Some(T),
    None,
}
// 使用 Option 枚举
// let some_number = Some(5);
// let some_string = Some("a string");
// let absent_number: Option<i32> = None;


fn main() {

    // 使用枚举的实例
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // let home1 = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback1 = IpAddr::V6(String::from("::1"));

    // let home2 = IpAddr2::V4(127, 0, 0, 1);
    // let loopback2 = IpAddr2::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    println!("Hello, world!");
}
