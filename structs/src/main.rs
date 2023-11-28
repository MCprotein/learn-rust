struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    /*
       구조체를 사용하려면 인스턴스를 생성해야한다.
    */
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    /*
       러스트에서는 특정 필드만 변경할 수 없다.
       변경하려면 인스턴스를 mutable하게 생성해야한다.
    */
    user1.email = String::from("anotheremail@example.com");

    /*
       기존 구조체 인스턴스로 새 구조체 인스턴스 생성하기
    */
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    /*
       자바스크립트의 speard operator 사용 가능
    */
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    /*
       튜플 구조체: 이름이 없고 필드 타입만 존재
       black과 origin은 서로 다른 타입이다.
       각각의 구조체는 고유의 타입이기 때문이다.
    */
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    /*
       유사 유닛 구조체: 필드 없는 구조체 -> 특정 타입의 트레잇을 구현해야 한다.
    */
}

fn build_user(email: String, username: String) -> User {
    /*
       자바스크립트와 동일하게 key, value가 같으면 생략할 수 있다.
    */
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
