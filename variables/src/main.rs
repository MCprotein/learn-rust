fn main() {
    /*
       상수: const
       mut 못쓴다.
       프로그램이 실행되는 동안 항상 유효하다.
    */
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    /*
       변수: let
       기본적으로 immutable하고, mut 키워드를 사용하여 mutable하게 만들 수 있다.
    */
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    /*
       shadow: 이미 선언한 변수와 같은 이름의 변수를 다시 선언할 수 있다.
    */
    let x = 10;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}
