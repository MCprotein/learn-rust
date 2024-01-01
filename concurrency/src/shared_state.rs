/*
   mutex 규칙
   1. 데이터를 사용하기 전에 락을 획득해야 한다.
   2. 데이터 사용이 끝나면 다른 스레드가 락을 얻을 수 있도록 락을 해제해야 한다.
*/

use std::sync::{Arc, Mutex};
use std::thread;

pub fn mutex() {
    let m = Mutex::new(5);

    {
        /*
        락을 획득한 후에는 뮤텍스 내부 값을 가변 참조처럼 취급할 수 있다.
        MutexGuard는 스마트 포인터이다. Deref, Drop이 모두 구현되어 있다.
        따라서 락 해제를 명시하지 않아도 스코프가 끝나면 자동으로 락이 해제된다.
        */
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    /*
       Rc<T>는 여기서 사용하지 못한다. 스레드간에 안전하게 공유되지 않기 때문이다.
       그저 호출될때마다 카운트를 더하고, 클론 값이 삭제될때마다 카운트를 빼기만 한다.
       따라서 동시성 이슈가 발생할 수 있다.

       이 상항에서는 Rc<T> 대신 Arc<T>를 사용한다. 원자적으로 참조 카운트된 타입이다.
       항상 Arc<T>를 사용하지 않는 이유는, 성능이 Rc<T>보다 좋지 않기때문이다.
       Arc<T>와 Rc<T>는 동일한 API를 가지고 있다.
    */
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
