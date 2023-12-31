pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've uesd up over 90% of your qouta!")
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your qouta!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));

            /*
               여러 소유자가 있는 가변 데이터를 관리하는 방법: RefCell<T> 와 Rc<T> 결합
               Rc<T>: 데이터를 여러 소유자가 가질 수 있지만, 불면참조만 제공
               RefCell<T>:

            */
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(msg));
            two_borrow.push(String::from(msg));
        }
    }

    /*
       RefCell<T>는 불변/가변 참조를 생성할때 &, &mut 대신 borrow, borrow_mut 메소드를 사용한다.
       둘 다 Deref 구현한다.
       borrow() -> Ref<T>
       borrow_mut() -> RefMut<T>

       RefCell는 현재 활성상태인 Ref<T>, RefMut<T> 스마트 포인트 개수 추적한다.
       borrow 메소드를 호출할때마다 RefCell<T>는 활성된 불변 참조의 수를 증가시킨다. drop되면 감소시킨다.
       RefCell<T>는 컴파일 대여 규칙과 마찬가지로 여러 개의 불변 참조 혹은 하나의 가변 참조를 가질 수 있다.
       이것을 위반하면 패닉이 발생한다.

    */
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
