#[derive(Debug)] // 这样可以立刻看到州的名称

// if let 获取通过等号分隔的一个模式和一个表达式 (if let 模式 = 表达式)。
// 它的工作方式与 match 相同，这里的表达式对应 match 而模式则对应第一个分支。
// 可以认为 if let 是 match 的一个语法糖，它当值匹配某一模式时执行代码而忽略所有其他值。


enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


fn main() {
    let some_u8_value = Some(3u8);
    // 如下示例, 当值为3时才执行代码
    // match some_u8_value {
    //     Some(3) => println!("three"),
    //     _ => (),
    // }

    // if let 优化后
    if let Some(3) = some_u8_value {
        println!("three");
    }



    let coin = Coin::Penny;
    let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
