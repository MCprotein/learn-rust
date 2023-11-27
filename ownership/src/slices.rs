pub fn slices() {
    /*
     스트링 슬라이스는 String의 일부분을 참조하는 참조자이다.
     스트링 슬라이스는 &str로 표현한다.
     스트링 슬라이스는 러스트의 중요한 데이터 타입이다.
     스트링 슬라이스는 String의 일부분을 참조하기 때문에, String의 소유권을 가지지 않는다.
     스트링 슬라이스는 불변 참조자이다.
    */
    /*
     s는 &str 타입이다. 이것은 불변참조자이기때문에 왜 스트링리터럴이 불변인지 설명 할 수 있다.
    */
    let s = "hello, world";

    /*
     슬라이스는 소유권을 가지지 않는다.
     collection 전체가 아니라 연속된 일련의 요소들을 참조할 수 있게 해준다.
    */

    /*
     아래 구조에서는 s가 사라져버리면 기껏 구한 인덱스 (word 변수)가 쓸모가없어진다.
    */
    let mut s = String::from("hello world");
    let word = first_word(&s);
    /*
     clear함수는 String을 잘라낼 필요가 있기때문에 가변 참조자를 갖기위한 시도를 한다.
     하지만 이미 불변참조자가 있으므로 에러가 발생한다.
    */
    s.clear();
    // println!("{}", word);

    /*
     스트링 슬라이스
    */
    let mut s = String::from("hello world");
    let hello = &s[0..5];
    // 0부터 시작하는경우에는 0을 생략해도 된다.
    let hello = &s[..5];
    let slice = &s[3..s.len()];
    // 끝까지 참조하고 싶을때는 끝 숫자를 생략해도 된다.
    let slice = &s[3..];
    // 그냥 복사하고 싶으면 숫자를 전부 생략한다.
    let slice = &s[..];
    let refref: &str = &s;
}

/*
  스트링을 입력받아 스트링의 첫 번째 단어를 반환하는 함수
  소유권이 없어서 스트링의 일부에 대해 표현할 방법이 없다.
  따라서 인덱스를 반환해야한다.

  &str은 &String의 일부를 참조하기때문에 &String을 받을 수 있다.
*/
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    /*
     enumerate() 메서드는 컬렉션의 해당 요소의 인덱스와 요소값을 튜플로 감싸서 반환한다.
    */
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // return i;
            return &s[0..i];
        }
    }

    &s[..]
}
