fn main() {
    // 新建一个空的 String
    let mut s = String::new();


    // 使用 to_string 方法从字符串字面量创建 String
    let data = "initial contents";
    let s2 = data.to_string();
    // 该方法也可直接用于字符串字面量：
    let s3 = "initial contents".to_string();


    // 使用 String::from 函数创建 String,  String::from 和 to_string 最终做到了完全相同的事情
    let hello = String::from("Hello");


    let mut s4 = "Hello".to_string();
    s4.push_str(", world");
    s4 += "!!!";
    println!("{}", s4);
    

    let mut s5 = String::from("foo");
    let s6 = "bar";
    s5.push_str(s6);
    println!("s6 is {}", s6);


    let mut s7 = String::from("lo");
    s7.push('l');
    println!("s7 is {}", s7);


    let s8 = String::from("Hello, ");
    let s9 = String::from("world!");
    // + 运算符使用了 add 函数 (fn add(self, s: &str) -> String {} )
    // s 参数：只能将 &str 和 String 相加
    // &String 会强转为 &str (解引用强制转换)
    let s10 = s8 + &s9; // s8 的所有权已转移给 s10, 后续不能在使用
    println!("s10 is {}", s10);



    // 多个字符串
    // format! 宏   不会获取任何参数的所有权
    let s11 = String::from("tic");
    let s12 = String::from("tac");
    let s13 = String::from("toe");
    // let s14 = s11 + "-" + &s12 + "-" + &s13;
    let s14 = format!("{}-{}-{}", s11, s12, s13);
    println!("s14 is {}", s14);



    // 索引字符串
    // String 不允许索引, 因为 String 存储的是字节而不是字符.
    // String 是一个 Vec<u8> 的封装。
    let s15 = String::from("hello"); // 每个标量值1个字节
    // let s15 = String::from("Здравствуйте"); // 每个标量值2个字节
    // let h = s15[0];
    // println!("h is {}", h);





    // 遍历字符串的方法
    // chars 方法会将其分开并返回六个 char 类型的值
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // bytes 方法返回每一个原始字节
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
