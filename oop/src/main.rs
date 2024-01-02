use oop::{Button, Screen, SelectBox};

/*
Duck typing
오리처럼 행동하면 오리로 봐도 된다.
객체의 변수와 메소드의 집합이 객체의 타입을 결정한다.
 */

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    let screen = Screen {
        components: vec![Box::new(String::from("Hi"))],
    };

    screen.run();
}
