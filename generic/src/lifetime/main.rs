/*
  러스트에서 모든 참조자는 lifetime을 갖는데, 이것은 해당 참조자가 유효한 scope 이다.
  대부분의 경우에서 lifetime은 암묵적으로 추론된다.

  lifetime 규칙
  1. 참조자인 각각의 파라미터는 그들 각각의 lifetime 파라미터를 가진다.
  2. 참조자가 하나만 있다면, 그 참조자의 lifetime은 반환값의 lifetime과 같다.
  3. 인자중에 &self나 &mut self가 있다면, self의 라이프타임이 반환값의 lifetime과 같다.
*/

/*
  lifetime은 dangling reference를 방지한다.
*/
pub fn borrow_checker() {
    let r;

    {
        let x = 5;
        r = &x;
        println!("r: {r}");
    }
}

pub fn generic_lifetime() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result)
}

/*
  x, y는 같은 라이프타임을 가진다고 명시한다.
  이 함수에 의해 반환되는 참조자의 라이프타임은 인자로 받은 참조자의 라이프타임 중 작은쪽과 같다.
  반환 타입의 라이프타임 파라미터는 인자 중 하나의 라이프타임 파라미터와 같다.
  같지 않을때는 함수 내에서 생성된 값을 반환하는 경우인데, 이 값은 함수가 끝나는 시점에 스코프 밖으로 벗어나서
  dangling reference가 되기 때문이다.
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn longest_can_not_compile<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str();
// }

/*
  서로 다른 구체적인 라이프타임을 가진 String 값의 참조자들 사용
*/
fn generic_lifetime_can_compile() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

fn generic_lifetime_can_not_compile() {
    let string1 = String::from("long string is long");
    // let result;
    {
        let string2 = String::from("xyz");
        // result = longest(string1.as_str(), string2.as_str());
    }
    // println!("The longest string is {}", result);
}
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

pub fn struct_lifetime() {
    let novel: String = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(".").next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

/*
 라이프타임 생략
*/
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

/*
  static lifetime
  'static 라이프타임은 프로그램 전체 생애주기를 가리킨다.
  모든 스트링 리터럴의 라이프타임은 'static 이다.
*/

use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
