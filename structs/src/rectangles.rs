/*
  println! 매크로에서 {}는 println!에게 Display 트레잇 포맷팅을 사용하라고 전달한다.
  지금까지 봐온 타입들은 Display가 기본적으로 구현되어 있는데, 보여주는 방법이 한 가지이기때문이다.
  하지만 구조체는 다양한 방법으로 보여줄 수 있기때문에 Display 트레잇을 구현해야한다.
  {:?}는 디버깅을 위한 표현식이지만, 출력을 위해 문자열로 변환한다.
  {:?}는 std::fmt::Debug 트레잇을 구현한 타입에 대해서만 사용할 수 있다.
*/
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

pub fn rectangles() {
    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect1)
    );
    println!("rect1 is {:#?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}
