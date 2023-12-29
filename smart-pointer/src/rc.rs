#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::rc::List::{Cons, Nil};
use std::rc::Rc;
pub fn rc() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // Rc::clone은 참조 카운트만 증가시키고 깊은복사하지 않아 성능이 좋다.
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}

pub fn rc_count() {
    // weak_count도 있다.
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
