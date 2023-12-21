use std::env;
use std::process;

use minigrep::Config;
fn main() {
    /* collect는 반복자가 가진 모든 값들을 벡터 형태로 반환한다.
       많은 종류의 콜렉션들이 사용될 수 있기때문에 String타입이라고 명시한다.
       dbg! = 디버그 매크로

       args[0] = 실행 파일의 경로와 이름
       args[1] = 첫번째 인수
    */

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        /*
           println!: stdout
           eprintln!: stderr
        */
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
