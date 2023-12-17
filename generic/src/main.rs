mod data_type;
mod lifetime;
mod traits;

use crate::traits::main::Summarizable3;

use self::data_type::main::{largest_char, largest_i32, Point};
use self::lifetime::main::struct_lifetime;
use self::traits::main::{largest, Pair, Summarizable, Summarizable2, Tweet};
/*
    제네릭은 monomorphization이라는 과정을 통해 컴파일 타임에 구체화된다.
    따라서 타입을 명시적으로 정의하는것과 비교했을때 성능 차이가 없다.
*/
fn main() {
    // date_type_run();
    // traits_run();
    lifetime_run();
}

fn date_type_run() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&chars);
    println!("The largest char is {}", result);

    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
    println!(
        "x: {}, y: {}",
        integer_and_float.get_x(),
        integer_and_float.get_y()
    );
    println!("{}", both_float.distance_from_origin());
}

fn traits_run() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());
    println!("1 new tweet: {}", tweet.summary2());
    println!("1 new tweet: {}", tweet.summary3());

    let numbers = vec![1, 2, 3, 4, 5];
    let largest_result = largest(&numbers);
    println!("{largest_result}");

    let pair = Pair::new(1, 2);
    pair.cmp_display();
    let pair = Pair::new("@", "bb21");
    pair.cmp_display();
}

fn lifetime_run() {
    struct_lifetime();
}
