pub trait Summarizable {
    fn summary(&self) -> String;
}

pub trait Summarizable2 {
    fn summary2(&self) -> String {
        String::from("(Read more...)")
    }
}

pub trait Summarizable3 {
    fn author_summary(&self) -> String;

    fn summary3(&self) -> String {
        format!("(Read more from {}...)", self.author_summary())
    }
}

// pub trait Display {
//     fn display(&self) -> String {
//         String::from("Display: ")
//     }
// }

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

/*
  Trait에 정의한 기본동작이 실행된다.
*/
impl Summarizable2 for Tweet {}

impl Summarizable3 for Tweet {
    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }
}

/*
    + 의미: Summarizable와 Display를 모두 구현한 타입만 사용할 수 있다.

    pub fn notify<T: Summarizable + Display>(item: T) {
    pub fn notify(item: &(impl Summarizable + Display)) {
    println!("Breaking news! {}", item.summary());
}

*/

/*
    아래 두 함수는 같다.

    fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}
    fn some_function<T, U>(t: T, u: U) -> i32
        where
        T: Display + Clone,
        U: Clone + Debug,
    {
    }
*/

/*
    > 연산자는 core::cmp::PartialOrd 기본 메소드이다.
*/
pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/*
    trait 구현체를 반환하는 함수
    이 함수를 호출하는 코드에서는 Summarizable의 구현체를 원할 뿐,
    이게 Tweet인지 NewsArticle인지 알 필요가 없고 알 수도 없다.
*/
fn returns_summarizable() -> impl Summarizable {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably arleady know, people"),
        reply: false,
        retweet: false,
    }
}

use std::fmt::Display;

pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            return println!("The largest member is x = {}", self.x);
        }

        println!("The largest member is y = {}", self.y);
    }
}
