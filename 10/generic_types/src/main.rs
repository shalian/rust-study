fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}



// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
    
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }



struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}


// 枚举定义中的泛型
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}


// Point<f32> 类型会有一个方法 distance_from_origin，
// 而其他 T 不是 f32 类型的 Point<T> 实例则没有定义此方法。
impl Point<f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


// 用 self 的 Point2 类型的 x 值（类型 T）和参数的 Point2 类型的 y 值（类型 W）来创建一个新 Point2 类型的实例
impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}



fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    // println!("The largest number is {}", largest(&number_list));
    assert_eq!(result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    // println!("The largest char is {}", largest(&char_list));
    assert_eq!(result, 'y');



    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // 字段 x 和 y 的类型必须相同，因为他们都有相同的泛型类型 T
    // let wont_work = Point { x: 5, y: 4.0 };

    // 不会报错
    let integer_and_float = Point2 { x: 5, y: 4.0 };



    let p = Point { x:5, y:10 };
    println!("p.x = {}", p.x());

    // the method was found for `Point<f32>`
    // println!("p.distance_from_origin = {}", p.distance_from_origin());


    //                 i32   f64
    let p1 = Point2 { x: 5, y: 10.4 };
    //                slice类型    char类型
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}", p3.x);
    println!("p3.y = {}", p3.y);




    let integer = Some(5);
    let float = Some(5.0);
    // 单态化编译后如下：

    enum Option_i32 {
        Some(i32),
        None,
    }
    enum Option_f64 {
        Some(f64),
        None,
    }
    // let integer = Option_i32::Some(5);
    // let float = Option_f64::Some(5.0);
}
