use std::io;

fn main() {
    // 元组
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // tup.0; // 500
    // let (x, y, z) = tup;
    // println!("x = {}, y = {}, z = {}", x, y, z);

    // 数组
    // let a: [i32; 5] = [1, 2, 3, 4, 5]; // i32 是每个元素的类型。分号之后，数字 5 表明该数组包含 5 个元素。
    // let a = [3; 5]; // [3, 3, 3, 3, 3]

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}