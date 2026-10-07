#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl 块中的所有内容都将与 Rectangle 类型相关联。
// impl Rectangle {
//     // 使用 &self（self: &Self） 来替代 rectangle: &Rectangle
//     // Self 类型是 impl 块的类型的别名
//     // 如果想要在方法中改变调用方法的实例，需要将第一个参数改为 &mut self
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     // 方法名可以与结构中的某一字段名称一样
//     fn width(&self) -> bool {
//         self.width > 0
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );

//     if rect1.width() {
//         println!("The rectangle has a nonzero width; it is {}", rect1.width);
//     }
// }



// 带有更多参数的方法
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// 每个结构体可以存在多个 impl 块
impl Rectangle {
    // 不以 self 为第一参数的关联函数，因此不是方法
    // 使用结构体名和 :: 语法来调用，比如 let sq = Rectangle::square(30);
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("The square rectangle is {:?}", Rectangle::square(5));
}