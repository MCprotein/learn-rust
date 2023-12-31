use std::thread;
use std::time::Duration;

fn main() {
    /*
       thread::spawn으로 새로운 스레드를 생성한다.
    */
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            /*
                스레드를 짧은 시간 동안 중지시키고 다른 스레드가 실행되도록 한다.
                스레드 스케줄링은 os에 의존한다.
            */
            thread::sleep(Duration::from_millis(1));
        }
    });
    // handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    /*
       메인스레드가 종료되어 생성된 스레드가 일찍 종료되고 스레드 실행 순서 보장이 안된다는 문제가 있다.
       이것을 해결하기 위해 thread::spawn의 반환 값을 변수에 저장한다.
       thread::spawn의 반환 타입은 JoinHandle이다.
       JoinHandle은 join 메소드를 호출했을때 스레드가 끝날때까지 기다릴 수 있는 소유된 값이다.

        join()을 호출하면 현재 실행중인 스레드가 join한 스레드가 종료될때까지 차단된다. 즉, 스레드가 끝날때까지 blocking 한다.
        스레드 차단: 작업을 수행하거나 종료되는것 방지
    */
    handle.join().unwrap();
}
