fn main() {
    /*
    match arms
     */
    let x = Some(5);

    let result = match x {
        None => None,
        Some(i) => Some(i + 1),
        _ => Some(1),
    };
    println!("{:?}", result);

    /*
    if let
     */
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color} as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
        /*
        아래 조건문은 가능하지 않다.
        else if let Ok(age) = age && age > 29 {}
        age는 {} 밖에서는 유효하지 않기 때문이다.
         */
    } else {
        println!("Using blue as the background color");
    }

    /*
    while let
     */
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    /*
    for loops
     */
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is a index {}", value, index);
    }

    let (x, y, z) = (1, 2, 3);

    /*
    function parameters
     */
    fn foo(x: i32) {}
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }
    let point = (3, 5);
    print_coordinates(&point);
}
