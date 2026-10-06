#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}



fn main() {
    let width = 30;
    let height = 50;
    let area = area(width, height);
    println!("The area is {}", area);


    let rect1 = (30, 50);
    let area2 = area2(rect1);
    println!("The area of the rect1 is {}", area2);


    // 结构体重构
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    let area3 = area3(&rect2);
    println!("The area of the rect2 is {}", area3);
    println!("rect2 is {:?}", rect2);


    // 结构体重构
    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(scale * 30), // dbg! 返回表达式的值的所有权，width 获得相同的值
        height: 50,
    };
    // 不希望 dbg! 有rect2 的所有权，传引用
    dbg!(&rect2); // dbg! 宏接收一个表达式的所有权，打印出代码中调用 dbg! 宏时所在的文件和行号，以及该表达式的结果值，并返回该值的所有权。
}


fn area(width: u32, height: u32) -> u32 {
    width * height
}


// 元组重构
fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}


// 结构体重构
fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}