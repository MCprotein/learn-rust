pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    #[ignore]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller))
    }

    #[test]
    #[ignore]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger))
    }

    #[test]
    #[ignore]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    #[ignore]
    fn greeting_contatins_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was {}",
            result
        )
    }

    #[test]
    #[ignore]
    // #[should_panic] // panic 발생해야 테스트 성공, 단순히 panic이 발생했는지만 판단하기때문에 우리가 의도하지 않은 panic일수도 있음
    #[should_panic(expected = "Guess value must be between 1 and 100")] // panic 발생해야 테스트 성공, panic 메시지가 포함되어있는지도 추가로 검증
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[ignore]
    /*
       아래처러 Generic type을 사용하는 경우, 반환값.is_err() 메소드로 검증한다.
       assert!(value.is_err())
    */
    fn generic_test() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    /*
       이름으로 테스트 하위 집합 실행
        cargo test add 라고 하면 함수이름에 add가 포함된 테스트만 실행한다. 중간이여도 상관없음
    */
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }
    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }
    #[test]
    fn one_hndred() {
        assert_eq!(102, add_two(100));
    }
}

/*
    무시한 테스트만 실행하고 싶으면 아래 명령어 실행
    cargo test -- --ignored
*/
