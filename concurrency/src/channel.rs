use std::sync::mpsc;
use std::thread;
use std::time::Duration;
/*
    mpsc::channel 함수를 사용하여 새로운 채널을 생성한다.
    mpsc: multiple producer, single consumer
    튜플 반환
    - 첫 번째 요소: transmitter (tx)
    - 두 번째 요소: receiver (rx)
*/
pub fn channel() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        /*
        값이 다른 스레드로 이동한 후여서 컴파일 에러가 발생한다.
        send 메소드는 매개변수의 소유권을 가져간다.
         */
        // println!("val is {}", val);
    });

    /*
       tx가 닫히면 rx.recv() 는 에러를 발생시킨다.
       rx.try_recv()는 Result<T, E>를 반환한다.
    */
    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
