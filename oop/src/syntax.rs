pub fn syntax() {
    /*
    match 스코프 내부에서 정의한 변수가 이미 스코프 밖에 있다면 바깥 변수는 가려진다.
    이것을 해결하기 위해 MatchGuard를 사용한다.
     */

    /*
    Multiple Patterns
    여러 조건 가능
     */
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    /*
    범위 조건 가능
    char, 숫자만 가능
    */
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    /*
    Destructuring Structs
    */
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})")
        }
    }
}
