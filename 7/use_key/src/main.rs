use std::collections::HashMap;
use std::fmt;
// use std::fmt::Result;
use std::io;
// 用 as 指定一个新的本地名称或者别名
// use std::io::Result as IoResult; 

// use std::cmp::Ordering;
// use std::io; //  use std::{cmp::Ordering, io};


// use std::io;
// use std::io::Write;   // use std::{self, Write};


// 通过 glob 运算符将所有的公有定义引入作用域
use std::collections::*;

fn function1() -> fmt::Result {
    println!("function1");
    Ok(())
}

fn function2() -> io::Result<()> {
    println!("function2");
    Ok(())
}


fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("{:?}", map);
}