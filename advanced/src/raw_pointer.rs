pub fn raw_pointer() {
    let mut num = 5;

    /*
    불변 참조와 가변 참조를 동시에 만드는게 원래는 안되지만 raw pointer를 사용하면 가능하다.
     */
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;

    println!("{}, {}", &num, *&num);
    println!("{r1:?} {r2:?} {address} {r:?}");

    unsafe {
        println!("{}", *r1);
        println!("{}", *r2);
    }

    unsafe {
        dangerous();
    }
}

unsafe fn dangerous() {}
