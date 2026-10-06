// fn main() {
//     let number = 3;

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }


fn main() {
    let condition = false;
    // 在 let 语句中使用 if 表达式
    // if 和 else 分支必须返回相同的类型，类型不同会报错
    let a = if condition { 5 } else { 6 };
    println!("a = {}", a);
}