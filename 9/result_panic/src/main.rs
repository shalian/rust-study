use std::fs::File;
use std::io::ErrorKind;

use std::io
use std::io::Read;
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     // Err(error) => panic!("Error: {}", error)

    //     // 匹配不同类型的错误
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Error: {}", e),
    //         },
    //         other_error => panic!("Error: {}", other_error),
    //     }
    // };


    // 高级写法
    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });




    // 失败时 panic 的简写：unwrap 和 expect
    // unwrap 返回 Ok 的值，如果 Err，则调用 panic!
    // let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Problem opening the file");





    // 传播错误
    
}



fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => panic!("Error: {}", error),
    };


    let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(error) => Err(error),
    // };

    
    // 使用 ? 运算符向调用者返回错误的函数
    // 与上述 match 表达式有着完全相同的工作方式
    // 结尾的 ? 将会把 Ok 中的值返回给变量 f
    // 如果是 Err，? 运算符会提早返回整个函数并将一些 Err 值传播给调用者
    // f.read_to_string(&mut s)?;
    // Ok(s)


    // 简化写法
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}