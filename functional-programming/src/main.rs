use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        /*
           클로저 함수는 변수 형태로 저장할 수 있다.
            타입 있어도 되고 없어도 된다.
        */
        let _expensive_closure = |num: u32| -> u32 {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        };

        fn add_one_v1(x: u32) -> u32 {
            x + 1
        }
        let add_one_v2 = |x: u32| -> u32 { x + 1 };
        let add_one_v3 = |x| x;
        let add_one_v4 = |x| x;
        let s = add_one_v3(String::from("hello"));
        let n = add_one_v4(5);

        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    /*
       클로저 불변참조
    */
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {:?}", list);
    /*
       클로저 가변참조
       이때는 클로저 정의와 호출 사이에 다른 소유권 빌림이 허용되지 않는다.

       클로저가 사용하는 값의 소유권을 빌리고 싶으면 클로저의 매개변수 앞에 move 키워드를 사용하면 된다.
       move는 클로저를 새 스레드에 전달하여 데이터를 이동시킨다.
       메인 스레드가 데이터의 소유권을 유지하지만, 새 스레드보다 먼저 끝나서 리스트가 버려지면 새 스레드의 참조는 무효화 될 것이다.
       따라서 컴파일러는 값이 새 스레드에 제공된 클로저로 이동되어 참조가 유효하게 되도록 요구한다.

    FnOnce
    한 번만 호출할 수 있는 클로저에 적용된다.
    모든 클로저는 적어도 이 트레이트를 구현한다.
    본문에서 값을 이동시키는 클로저는 FnOnce만 구현하며 다른 Fn 트레이트는 구현하지 않는다.
    왜냐하면 그 클로저는 한 번만 호출될 수 있기 때문.

    FnMut
    본문에서 값을 이동시키지 않지만 캡처된 값을 변이시킬 수 있는 클로저에 적용된다.
    이러한 클로저는 여러 번 호출될 수 있다.

    Fn
    본문에서 값을 이동시키지 않고 캡처된 값을 변이시키지 않거나 환경에서 아무 것도 캡처하지 않는 클로저에 적용된다.
    이러한 클로저는 환경을 변이시키지 않고 여러 번 호출될 수 있다.
    병렬로 클로저를 여러 번 호출하는 경우에 중요하다.



    */
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();

    println!("After calling closure: {:?}", list);

    // let list = vec![1, 2, 3];
    // println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // println!("{:?}", list);

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    /*
    아래 코드는 value(String)을 한 번 sort_operations으로 넣고 나면 더 이상 환경에 남아있지 않는다.
    따라서 FnMut 구현이 안되어있고 fnOnce만 구현이 되어있어서 컴파일 에러가 발생한다.
     */
    // let mut sort_operations = vec![];
    // let value = String::from("by key called");

    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}
