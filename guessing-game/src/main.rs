/*
    외부에 의존하는 크레이트가 있다는 것을 의미
    use rand로 표기할 수도 있고, rand::로 rand의 모든것을 가져올 수 있다.
*/
extern crate rand;

/*
    Rng는 정수 생성기가 구현한 메소드들을 정의한 trait이며 해당 메소드를 이용하기 위해서 반드시 스코프 내에 있어야한다.
*/
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    /*
        rand::thread_rng()는 현재 실행중인 OS의 적절한 시드값을 사용하여 초기화된 난수 생성기를 반환한다.
        gen_range는 Rng trait의 메소드이다.
        1~101 사이의 난수를 생성한다.
    */
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        /*
           rust에서 변수는 기본적으로 불변이다.
           mut 키워드를 사용하여 가변 변수를 만들 수 있다.
           new(): 생성자
           ::는 new가 String의 연관함수라는 것을 의미한다.
           연관함수는 특정 타입의 인스턴스를 생성하지 않고도 호출할 수 있는 함수이다. = static 메소드


        */
        let mut guess = String::new();

        /*
           std::io를 use (import)하지 않는다면 아래 코드에서 std::io::stdin() 으로 작성해야한다.
           stdin()은 표준 입력 핸들에 대한 인스턴스를 반환한다.
           read_line()은 입력받은 문자열을 인자로 받은 변수에 저장한다.
           &는 go에 있는거랑 똑같은듯?

           read_line은 io::Result 타입을 반환한다.
           Result 타입은 enum이다. Ok와 Err 두 가지의 값을 가질 수 있다.
           expect는 에러가 있으면 인자로 받은 메세지를 출력하고 프로그램을 종료한다. Ok이면 성공한 값을 반환한다.
        */
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        /*
           러스트는 변수를 또 선언할때 먼저 선언한건 가려진다
           이건 보통 하나의 값에서 타입만 바꾸고 싶을때 사용한다.
           guess_str, guess_int 이렇게 안하려고.

           turbofish는 제네릭 타입을 명시할때 사용한다.
           ::<> 로 표기하다.
        */
        let guess = guess.trim().parse::<u32>().expect("Please type a number!");

        /*
           JavaScript의 template literal과 비슷
        */
        println!("You guessed: {}", guess);

        /*
           cmp 메소드는 두 값을 비교한다. 여기서는 guess와 secret_number
           cmp 메소드는 Ordering (enum) 타입을 반환한다.
            switch case와 비슷
        */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
