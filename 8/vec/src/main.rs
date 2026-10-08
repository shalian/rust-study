fn main() {
    // let v = vec![1, 2, 3];
    // println!("{:?}", v);

    // 使用 push 方法向 vector 增加值
    // let mut v = Vec::new();
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // println!("{:?}", v);

    {
        let v = vec![1, 2, 3];
    } // v 离开作用域，vector 被丢弃，其元素也被丢弃

    let v = vec![1, 2, 3];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v[100]; // panic!
    // let does_not_exist = v.get(100); // None

    // 当获取了 vector 的第一个元素的不可变引用并尝试在 vector 末尾增加一个元素的时候，这是行不通的
    // 在 vector 的结尾增加新元素时，在没有足够空间将所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。
    // 这时，第一个元素的引用就指向了被释放的内存。
    // let first = &v[0];
    // v.push(4);



    // 遍历 vector 中的元素，每个元素的引用都赋值给变量 变量名
    let v2 = vec![100, 32, 57];
    for i in &v2 {
        println!("{}", i);
    }

    // 遍历可变 vector 的每一个元素的可变引用以便能改变他们
    let mut v3 = vec![100, 32, 57];
    for i in &mut v3 {
        // * 解引用，将可变引用转换为可变值
        *i += 50;
    }
    println!("{:?}", v3);


    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

// 使用枚举来储存多种类型
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}