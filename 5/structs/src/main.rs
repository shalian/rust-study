// 结构体
// 结构体的名字需要描述它所组合的数据的意义


struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 类单元结构体
struct AlwaysEqual;


fn main() {

    // 创建结构体实例，注意整个实例必须是可变的
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.username);


    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    // 打印user2的email
    println!("user2 email: {}", user2.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);



    // 
    let subject = AlwaysEqual;
}


fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
