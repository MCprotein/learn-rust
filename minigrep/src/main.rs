use std::env;
use std::fs;

fn main() {
    /* collect는 반복자가 가진 모든 값들을 벡터 형태로 반환한다.
       많은 종류의 콜렉션들이 사용될 수 있기때문에 String타입이라고 명시한다.
       dbg! = 디버그 매크로

       args[0] = 실행 파일의 경로와 이름
       args[1] = 첫번째 인수
    */
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
