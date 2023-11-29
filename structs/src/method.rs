/*
  메소드는 함수와 유사하다.
  fn 키워드, 파라미터와 반환값 등

  하지만 함수와는 다르게 구조체안에 정의된다. (혹은 enum이나 트레잇 객체 안에 정의)
  첫 번째 파라미터는 항상 self이며, self는 메소드를 호출한 구조체의 인스턴스를 가리킨다.
  class의 메소드와 유사하며, self는 this와 유사하다.

  impl 블록안에 정의한다.
  아래 두 코드는 동일하다. 러스트의 자동 참조 및 역참조 기능에 의한 것이다.
  p1.distance(&p2);
  (&p1).distance(&p2);
*/

/*
  연관 함수
  impl안에서 self 파라미터를 갖지 않는 함수
  메소드가 아니다.
  ex) String::from
  class의 static 메소드와 유사하다.
*/
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    /* &self: self readonly */
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            length: size,
            width: size,
        }
    }
}

pub fn method() {
    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    hold();

    /* 연관함수 (static 메소드와 비슷) */
    let sq = Rectangle::square(3);
    println!("sq: {:#?}", sq)
}

fn hold() {
    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };
    let rect2 = Rectangle {
        length: 40,
        width: 10,
    };
    let rect3 = Rectangle {
        length: 45,
        width: 60,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
