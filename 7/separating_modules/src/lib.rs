// 声明 front_of_house 模块，其内容将位于 src/front_of_house.rs
mod front_of_house;

pub use crate::front_of_house::hosting;

fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}