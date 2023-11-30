mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }
    pub mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

/*
    use는 hosting 모듈이 크레이트 루트에서 정의된것처럼 특정 스코프에만 해당하는 바로가기를 만든다.
    따라서 customer mod 내부에서는 hosting 모듈을 사용할 수 없다.
    사용하려면 super::hosting::add_to_waitlist(); 를 사용해야한다.

    부모 모듈까지만 작성해서 hosting::add_to_waitlist(); 처럼 사용하는게 관용적인 방법이다.
    하지만 구조체, 열거형 등과 같은것들은 전체 경로를 지정하는것이 관용적이다. ex) HashMap
    예외) std::fmt::Result, std::io::Result

    as로 별칭 지정 가능. node.js와 같다.

    pub use로 가져온 모듈을 다시 내보낼 수 있다.
    restautant::front_of_house::hosting::add_to_waitlist(); 를 restaurant::hosting::add_to_waitlist(); 로 사용할 수 있다.
*/
// use crate::front_of_house::hosting;
pub use crate::front_of_house::hosting::add_to_waitlist;
use std::collections::HashMap as MyHashMap;

pub fn eat_at_restaurant() {
    /*
       절대경로
       크레이트 이름을 사용하여 크레이트 루트(src/lib.rs)에서 시작
       일반적으로 코드 정의와 항목 호출을 독립시키기를 원하기때문에 절대 경로 선호됨
       front_of_house는 public하지 않지만, eat_at_restaurant와 동일한 모듈에 정의되어 있기 때문에 참조할 수 있다.
    */

    // crate::front_of_house::hosting::add_to_waitlist();
    // hosting::add_to_waitlist();
    add_to_waitlist();
    /*
       상대경로
       eat_at_restaurant 함수와 같은 모듈 내에서 시작
       모듈이름으로 시작하면 현재 모듈과 같은 위치에서 시작
    */
    front_of_house::hosting::add_to_waitlist();

    let mut map = MyHashMap::new();
    map.insert(1, 2);
}

mod customer {
    pub fn eat_at_restaurant() {
        // super::hosting::add_to_waitlist();
        super::add_to_waitlist();
    }
}
