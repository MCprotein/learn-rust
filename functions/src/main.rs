fn main() {
    /*
        rust는 변수, 함수 이름에 snake case를 사용한다.
        JavaScript와 같이 함수가 앞에 작성되지 않아도 된다. (like hoisting)
    */
    another_function(5, 6);

    /*
       구문: 어떤 동작을 수행하지만 값을 반환하지 않는다. ex) let x = 5
       let x = (let y = 6)에서 let y = 6은 구문이라서 반환값이 없다. 따라서 에러가 발생한다.

       표현식: 구문의 일부이다.
       종결을 나타내는 세미콜론을 사용하지 않는다.
    */
    let x = 5;
    let y = {
        let x = 3;
        x + 1 // 표현식
    };
    println!("x: {}, y: {}", x, y);

    let x = five();
    println!("The value of x is: {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}, y is: {}", x, y);
}

/*
    Rust에서 반환값은 함수 본문의 마지막 표현식의 값이다.
    return을 사용할 수도 있지만, 대부분은 마지막 표현식을 반환한다.
*/
fn five() -> i32 {
    5
}
