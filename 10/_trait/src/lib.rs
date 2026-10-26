use std::fmt::Display;


// trait 名称
pub trait Summary {
    // 声明描述实现这个 trait 的类型所需要的行为的方法签名
    fn summarize(&self) -> String; //  分号结尾
}


// 新闻结构体
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 实现 trait
impl Summary for NewsArticle { 
    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}



pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}


impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}



// 默认实现
pub trait Summary2 {
    fn summarize2(&self) -> String {
        format!("(Read more...)")
    }
}

// 可指定一个空的 impl 块
impl Summary2 for NewsArticle {}



// 默认实现允许调用相同 trait 中的其他方法，哪怕这些方法没有默认实现
pub trait Summary3 {
    fn summarize_author(&self) -> String;
    
    fn summarize3(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}


// 在实现 trait 时定义 summarize_author 即可
impl Summary3 for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}


// =========== trait 作为参数 ===========

pub fn notify(item: impl Summary3) {
    println!("Breaking news! {}", item.summarize3());
}


// 1. Trait Bound 语法，等价于 notify(item: impl Summary3)
pub fn notify2<T: Summary3>(item: T) {
    println!("Breaking news! {}", item.summarize3());
}


// 2. 通过 + 指定多个 trait bound
// pub fn notify3(item: impl Summary + Display) {}

// pub fn notify4<T: Summary + Display>(item: T) {}




// 3. 通过 where 简化 trait bound
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}

// 简化后，等价于 some_function
// fn some_function2<T, U>(t: T, u: U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {}




// 4. 返回实现了 trait 的类型
fn returns_summarizable() -> impl Summary3 {
    
    if true {
        // 返回 NewsArticle 会导致无法编译，返回值类型指定为返回 impl Summary3
        // NewsArticle {
        //     headline: String::from("Penguins win the Stanley Cup Championship!"),
        //     location: String::from("Pittsburgh, PA, USA"),
        //     author: String::from("Iceburgh"),
        //     content: String::from(
        //         "The Pittsburgh Penguins once again are the best \
        //          hockey team in the NHL.",
        //     ),
        // }
    }


    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}



// 5. 使用 trait bounds 来修复 largest 函数
// fn largest<T: PartialOrd>(list: &[T]) -> T {}


// 6. largest 函数改成使用泛型后，现在 list 参数的类型就有可能是没有实现 Copy trait 的
// 可以在 T 的 trait bounds 中增加 Copy
pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}



// 7. 使用 trait bound 有条件地实现方法
struct Pair<T> {
    x: T,
    y: T,
}
// 类型 Pair<T> 总是实现了 new 方法
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}
// 为 T 类型实现了 PartialOrd trait （来允许比较） 和 Display trait （来启用打印）的 Pair<T> 才会实现 cmp_display 方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}





// 8. 标准库为任何实现了 Display trait 的类型实现了 ToString trait
impl<T: Display> ToString for T {
    fn to_string(&self) -> String {
        format!("{}", self)
    }
}

// let s = 3.to_string();








fn main () {
    // 实现允许调用相同 trait 中的其他方法
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    // println!("1 new tweet: {}", tweet.summarize());
}