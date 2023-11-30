#[derive(Debug)]
pub enum UsState {
    Alabama,
    Alaska,
}
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

/*
  switch case와 비슷하다.
  if는 조건이 bool 타입이여야 하는데, match는 아무 타입이나 가능하다.
  바로 표현식 사용할거면 중괄호 안써도 된다.
  default는 _로 표현한다.
*/
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn main() {
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{}", plus_one(Some(3)).unwrap());
    // println!("{}", plus_one(None).unwrap());
    default_placeholder();
}

fn default_placeholder() {
    let some_u8_value = 9u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // _는 모든 값을 의미한다. ()는 아무것도 하지 않는 표현식이다.
    }
}
