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

pub fn eat_at_restaurant() {
    /*
       절대경로
       크레이트 이름을 사용하여 크레이트 루트(src/lib.rs)에서 시작
       일반적으로 코드 정의와 항목 호출을 독립시키기를 원하기때문에 절대 경로 선호됨
       front_of_house는 public하지 않지만, eat_at_restaurant와 동일한 모듈에 정의되어 있기 때문에 참조할 수 있다.
    */

    crate::front_of_house::hosting::add_to_waitlist();
    /*
       상대경로
       eat_at_restaurant 함수와 같은 모듈 내에서 시작
       모듈이름으로 시작하면 현재 모듈과 같은 위치에서 시작
    */
    front_of_house::hosting::add_to_waitlist();
}
