// 字符串 slice
// 由中括号中的 [starting_index..ending_index] 指定的 range 创建一个 slice
// 包含开始位置，不含结束位置
// 长度为 ending_index - starting_index

// 字符串字面量就是 slice
// let s = "Hello, world!";
// 这里 s 的类型是 &str：它是一个指向二进制程序特定位置的 slice。这也就是为什么字符串字面量是不可变的；&str 是一个不可变引用。


fn main() {
    let mut s1 = String::from("hello world");
    let word = first_word1(&s1); // hello

    // s1.clear();

    println!("{}", word);



    
    let s2 = String::from("hello world");

    let hello = &s2[0..5];
    let world = &s2[6..11]; // world 将是一个包含指向 s2 索引 6 的指针和长度值 5 的 slice。

    println!("hello = {}, world = {}", hello, world);



    // let s3 = String::from("hello");

    // let slice1 = &s3[0..2];
    // let slice1 = &s3[..2];


    // let s4 = String::from("hello");
    // let len = s4.len();

    // let slice2 = &s4[0..len];
    // let slice2 = &s4[..];



    // 其他类型 slice
    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..3];
    println!("{:?}", slice);
}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word1(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


// 字符串 slice 作为参数
fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
