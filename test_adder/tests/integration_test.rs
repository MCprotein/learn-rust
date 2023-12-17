use test_adder;

mod common;

/*
  cargo는 tests 디렉토리를 특별취급하여 cargo test를 실행할때에만 컴파일한다.
*/
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_adder::add_two2(2));
}
