// trait
// 一个类型的行为由其可供调用的方法构成。
// 如果可以对不同类型调用相同的方法的话，这些类型就可以共享相同的行为了。
// trait 定义是一种将方法签名组合起来的方法，目的是定义一个实现某些目的所必需的行为的集合。

use _trait::{ Summary,Summary2,Summary3, Tweet, NewsArticle,notify, largest };

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());



    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };

    println!("New article available! {}", article.summarize2());



    println!("tweet.summarize3: {}", tweet.summarize3());


    // trait 作为参数
    notify(tweet);




    let number =  largest(&[1, 2, 3]);  
    println!("number: {}", number);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
