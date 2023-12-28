use crate::List::{Cons, Nil};
// (1, (2, (3, Nil)))

/// 단순히 이렇게 쓰면 무한재귀여서 러스트 컴파일러는 얼마나 공간을 할당해야되는지 알 수 없어서 에러 발생시킨다.
/// Box<T>는 포인터라서 러스트 컴파일러는 항상 얼마만큼의 공간을 필요로하는지 알 수 있다. (포인터의 크기는 데이터 크기와 무관)
/// 이 방식을 간접 참조라고 한다.
/// Box<T> 타입은 Deref 트레이트를 구현하고 있어서 Box<T> 타입의 값을 사용하고자 할 때마다 러스트가 자동으로 * 연산자를 사용해서 값을 가져온다.
/// Box<T> 타입은 Drop 트레이트를 구현하고 있어서 Box<T> 타입의 값이 스코프 밖으로 벗어나면 러스트가 자동으로 drop 함수를 호출한다.
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
}
