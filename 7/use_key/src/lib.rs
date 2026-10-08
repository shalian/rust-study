mod front_of_house {
    // Rust 中默认所有项（函数、方法、结构体、枚举、模块和常量）都是私有的
    // 添加 pub 关键字, 使其在其他模块中也可以调用
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }
}

// use crate::front_of_house::hosting; // 绝对路径写法
// use front_of_house::hosting; // 相对路径写法

// 使用 pub use 重导出名称
pub use crate::front_of_house::hosting;

fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}