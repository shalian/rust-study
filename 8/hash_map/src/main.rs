
// hash map 所有的键必须是相同类型，值也必须都是相同类型。

fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);



    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // HashMap<_, _> 类型标注是必要的，因为 collect 有可能当成多种不同的数据结构
    // Rust 能够根据 vector 中数据的类型推断出 HashMap 所包含的类型
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores2);



    // 哈希 map 和所有权
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    // 移动后 哈希 map 会成为这些值的所有者
    map.insert(field_name, field_value);
    // 这里 field_name 和 field_value 不再有效，



    // 访问哈希 map 中的值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{} team score is {:?}", team_name, score);

    // for 循环遍历哈希 map
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }


    // 更新哈希 map 中的值
    // 插入相同键的值会覆盖掉旧值
    scores.insert(team_name, 100);
    println!("{:?}", scores);



    // 使用 entry 方法只在键没有对应一个值时插入
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);





    // ex: 根据旧值更新一个值
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // or_insert 方法事实上会返回这个键的值的一个可变引用（&mut V）
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
