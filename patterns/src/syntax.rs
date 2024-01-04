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

    /*
    Destructuring Enums
     */
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}");
        }
    }

    /*
    Destructuring Nested Structs and Enums
     */
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}")
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }

    /*
    Destructuring Structs and Tuples
     */
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    /*
    Ignoring an Entire Value with _
    */
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {y}");
    }

    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    /*
    Ignoring an Unused Variable by Starting Its Name with _
    변수 이름을 _로 시작할 경우 값을 변수에 바인딩 하지 않는다.
     */
    let _x = 5;
    let y = 10;

    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    /*
    Ignoring Remaining Parts of a Value with ..
     */
    struct Point3 {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3 { x: 0, y: 0, z: 0 };

    match origin {
        Point3 { x, .. } => println!("x is {x}"),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    /*
    tuple에서는 ..를 한 번만 사용할 수 있다.
    컴파일러가 튜플의 요소를 얼마나 생략해야되는지 모르기때문
     */
    // match numbers {
    //     (.., second, ..) => {
    //         println!("Some numbers: {}", second);
    //     }
    // }

    /*
    Extra Conditionals with Match Guards
    */
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {x:?}, y = {y}");

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    /*
    @ Bindings
    */
    enum Message4 {
        Hello { id: i32 },
    }

    let msg = Message4::Hello { id: 5 };

    match msg {
        Message4::Hello {
            // id의 값을 id_variable 변수에 바인딩
            id: id_variable @ 3..=7,
        } => println!("Found and id in range: {id_variable}"),
        // 이건 id의 값이 무엇인지 모른다.
        Message4::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message4::Hello { id } => println!("Found some other id: {id}"),
    }
}
