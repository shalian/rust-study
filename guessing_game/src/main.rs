use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("Game over!");
                break;
            },
        }
    }
}


// fn main() {
//     println!("Guess the number!");
//     println!("Please input your guess.");

//     // 定义一个变量 apples，将其初始化为 10，并且不可变。
//     // let apples = 10;
//     // 创建一个变量 guess，并分配一个 String 空字符串。变量名前添加 mut (mutability) 关键字表示这是一个可变的变量。
//     let mut guess = String::new(); 

//     // 从标准输入读取一行输入，并将其分配给 guess 变量。
//     // 如果读取失败，打印错误信息并退出程序。
//     io::stdin()
//         .read_line(&mut guess) // 从标准输入句柄中获取用户输入，并将其分配给 guess 变量。
//         .expect("Failed to read line");

//     println!("You guess: {}", guess); // 里面的 {} 是预留在特定位置的占位符，用于将 guess 变量的值插入到字符串中。
// }