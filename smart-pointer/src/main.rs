use crate::List::{Cons, Nil};
use std::ops::Deref;
// (1, (2, (3, Nil)))

/// 단순히 이렇게 쓰면 무한재귀여서 러스트 컴파일러는 얼마나 공간을 할당해야되는지 알 수 없어서 에러 발생시킨다.
/// Box<T>는 포인터라서 러스트 컴파일러는 항상 얼마만큼의 공간을 필요로하는지 알 수 있다. (포인터의 크기는 데이터 크기와 무관)
/// 이 방식을 간접 참조라고 한다.
/// Box<T> 타입은 Drop 트레이트를 구현하고 있어서 Box<T> 타입의 값이 스코프 밖으로 벗어나면 러스트가 자동으로 drop 함수를 호출한다.
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 역참조가 가능하도록 Deref trait 구현
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);

    // 일반 참조
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // assert_eq!(5, y); 숫자와 숫자의참조를 비교하는 것은 불가능하다.

    // Box는 스마트 포인터이기 때문에 일반 참조와는 다르게 *y를 하지 않아도 된다.
    // Box는 Deref 트레이트를 구현하고 있어서 * 연산자를 사용할 수 있다.
    // Box는 스마트포인터지만 일반참조처럼 사용할 수 있다.
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // 그냥 하면 MyBox를 어떻게 역참조해야되는지 알지 못해서 컴파일 에러가 발생한다.
    // 역참조를 활성화하려면 Deref 트레이트를 구현해야한다.
    // *y는 사실상 *(y.deref()) 과 같다.
    assert_eq!(5, *y);

    // 역참조 강제 변환
    // 예시 &String -> &str

    let m = MyBox::new(String::from("Rust"));
    // *m을 호출하면 deref 메소드가 호출되어 &self.0을 반환하는데, 이것은 &String::from("Rust")와 같다. 즉, *m는 m.deref()와 같다.
    // &m 호출해도 &self.0 반환. 처리 방식은 다르지만 결과는 같다
    println!("{:?}", *m);
    hello(&m);
    // 만약 역참조 강제 변환을 사용하지 않았다면 아래와 같이 해야한다.
    // hello(&(*m)[..]);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
