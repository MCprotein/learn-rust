use std::fmt::format;

pub fn str() {
    /*
     &str과 String의 차이점
     &str은 스트링 슬라이스이다.
     스트링 슬라이스는 String의 일부를 참조하는 참조자이다.
     스트링 슬라이스는 불변 참조자이다.
     스트링 슬라이스는 소유권을 가지지 않는다.
     스트링 슬라이스는 러스트의 중요한 데이터 타입이다.
     스트링 슬라이스는 String의 일부분을 참조하기 때문에, String의 소유권을 가지지 않는다.
     String은 힙에 저장되는데, 스트링 슬라이스는 스트링의 일부분을 참조하기 때문에 스트링 슬라이스는 스택에 저장된다.
    */
    let data = "initial contents";

    let s = data.to_string();

    println!("s: {s}");
    println!("data: {data}");
    let result = data == s;
    println!("{result}");

    let mut s = String::from("initial contents");
    let s2 = "bar";
    s.push_str(s2);
    println!("{s}");

    /*
     + 연산자를 사용하여 스트링을 합칠 수 있다.
      + 연산자는 add 메서드를 호출한다.
      fn add(self, s: &str) -> String {
      add 함수는 자기자신에 인자로 받은 s를 더한다.
      그런데 아래 &s2는 &String이지, &str이 아니다.
      그럼에도 불구하고 컴파일 에러가 발생하지 않는 이유는
      러스트가 역참조 강제변환(deref coercion)을 해주기 때문이다. &String -> &str
      즉, &s2를 &s2[..]로 변환해준다.
      그리고 add가 s1의 소유권을 가져가기 때문에 s1은 더이상 유효하지 않다.
      즉, s1의 소유권을 가져다가 s2의 복사본을 추가한다음 소유권을 반환하는것과 같다.
    */
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{s3}");
    /*
       여러 스트링을 더할때는 +가 불편하기때문에 format! 매크로를 이용한다.
       이 매크로는 참조를 사용하기때문에 소유권을 가지고 있지 않다.
    */
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s1}, {s2}, {s3}, {s}");

    /*
       String은 인덱싱을 지원하지 않는다.
       String은 Vec<u8>의 래퍼이기때문에 인덱싱을 지원하지 않는다.

       Здравствуйте를 UTF-8로 인코딩하면 24바이트가 된다.
       UTF-8로 인코딩하면 [0]을 가져올때 첫 번째 문자가 아니라 첫 번째 바이트를 가져온다. ex) 208
       [0]을 가져올때 첫 번째 문자를 가져오려면 [0..1]을 사용해야한다.

       String에 인덱스를 사용할 수 없는 또 다른 이유는, 언제나 O(1)을 기대하는데
       String 내부에 얼마나 많은 유효 문자가 있는지 확인하기 위해 언제나 값을 처음부터 훑어야 하기 때문이다.
    */
    let len = String::from("Hola").len();
    let hello = "Здравствуйте";
    let len = String::from(hello).len();
    let answer = &hello[..];
    println!("{answer}");

    /*
       문자열 슬라이스
       &hello[0..4];는 4바이트를 담고 있는 &str 이다.
       한 글자당 2바이트이므로 Зд가 될 것이다.

        [0..1]은 1바이트이므로 위 글자가 담기지 못해 panic이 발생한다.
    */
    let ss = &hello[0..2];
    println!("{ss}");
    /*
       아래처럼 하면 t가 1바이트여서 panic이 발생하지 않는다.
    */
    println!("{}", &"tac"[0..1]);

    // for c in "नमस्ते".chars() {
    // for c in "नमस्ते".bytes() {
    for c in "नमस्ते".bytes() {
        println!("{c}");
    }
}
