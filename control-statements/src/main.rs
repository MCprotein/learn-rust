fn main() {
    /*
       JavaScript의 Truthy, Falsy같은 개념은 없다.
       따라서 if 조건문 안에는 boolean 타입 값만 들어갈 수 있다.
    */

    let number = 5;

    if number < 5 {
        println!("condition was true");
    } else if number == 5 {
        println!("condition was equal")
    } else {
        println!("condition was false")
    }

    let condition = true;
    /*
        if 조건문은 표현식 (expression)이다.
        따라서 if 조건문의 결과값을 변수에 할당할 수 있다.
        코드 블록은 그 자체로 표현식이며 마지막에 위한 표현식을 산출한다.
        if와 else의 결과값 타입은 같아야 한다.
    */
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    loop {
        println!("again!");
        break;
    }

    /*
       while
    */
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    /*
       collection 반복
       while과 for
    */
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    /*
       rev() = reverse()
    */
    for number in (1..4).rev() {
        println!("{}!", number)
    }
    println!("LIFTOFF!!!");
}
