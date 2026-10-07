#[derive(Debug)] // 这样可以立刻看到州的名称
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



// match 与 枚举 相结合使用
fn value_in_cents(coin: Coin) -> u8 {
    // match 可以返回任何类型，if 表达式必须返回布尔值
    // 每个分支相关联的代码是一个表达式，而表达式的结果值将作为整个 match 表达式的返回值。
    match coin {
        Coin::Penny => {
            println!("Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter {:?}", state);
            // 这里可以添加根据州的奖励逻辑
            25
        },
    }
}


// 有值则加 1，没有值则返回 None
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


fn main() {
    let cents = value_in_cents(Coin::Penny);
    let cents2 = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("cents = {}", cents);
    println!("cents2 = {}", cents2);


    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six = {:?}", six);
    println!("none = {:?}", none);



    // 通配模式和 _ 占位符
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),  // 可以使用 _ 占位符来替代变量 other
        // _ => (),
    }
}


fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}