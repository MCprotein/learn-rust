#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

/*
    Quit: 연관된 데이터가 없음
    Move: 익명 구조체 -> struct 키워드 안씀
    Write: String
    ChangeColor: 세 개의 i32
*/
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    /*
        enum에 메소드를 구현할 수 있다.
        self는 enum의 인스턴스를 의미한다.
    */
    fn call(&self) {
        println!("{:?}", self)
    }
}

/*
    러스틑에는 null이 없지만 값의 존재/부재를 나타낼 수 있는 열거형이 있다.
    기본적으로 포함되어 있어서 명시적으로 가져오지 않아도 사용할 수 있다.  Some과 None도 마찬가지
    None은 Null과 같은 의미이다.
    enum Option<T> {
        Some(T),
        None,
    }

    Option<T>는 T 타입이 아니여서 T에 대한 연산을 수행하려면 T로 변환해야한다.
    ex) Option<i32> + i32는 불가능하다. (컴파일에러 발생)
*/

pub fn definition() {
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}, {:?}", home, loopback);

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    println!("{:?}, {:?}, {:?}", some_number, some_string, absent_number);
    println!("{}", some_number.unwrap());
}
