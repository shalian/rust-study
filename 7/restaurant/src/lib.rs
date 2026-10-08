pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


// 可以在模块中包含其他模块
mod front_of_house {
    // Rust 中默认所有项（函数、方法、结构体、枚举、模块和常量）都是私有的
    // 添加 pub 关键字, 使其在其他模块中也可以调用
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}


fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}



// 使用 super 起始的相对路径
// fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         // fix_incorrect_order 函数在 back_of_house 模块中，所以我们可以使用 super 进入 back_of_house 父模块，也就是本例中的 crate 根。
//         super::serve_order();
//     }

//     fn cook_order() {}
// }



// 创建公有的结构体和枚举
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // 如果我们将枚举设为公有，则它的所有成员都将变为公有。
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }


    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant2() {
    // 在夏天点一份黑麦面包作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 更改我们想要的面包
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 如果取消下一行的注释，将会导致编译失败；我们不被允许
    // 看到或更改随餐搭配的季节水果
    // meal.seasonal_fruit = String::from("blueberries"); // 私有字段



    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!("{:?}", order1);
    println!("{:?}", order2);
}