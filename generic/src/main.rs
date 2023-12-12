mod data_type;

use self::data_type::main::{largest_char, largest_i32, Point};
/*
    제네릭은 monomorphization이라는 과정을 통해 컴파일 타임에 구체화된다.
    따라서 타입을 명시적으로 정의하는것과 비교했을때 성능 차이가 없다.
*/
fn main() {
    date_type_run();
}

fn date_type_run() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&chars);
    println!("The largest char is {}", result);

    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
    println!(
        "x: {}, y: {}",
        integer_and_float.get_x(),
        integer_and_float.get_y()
    );
    print!("{}", both_float.distance_from_origin());
}
