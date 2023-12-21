vscode 설정
https://code.visualstudio.com/docs/languages/rust#_formatting

format on save 설정

```json
// settings.json (user)
"[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave": true
  }
```

공식문서
https://rinthel.github.io/rust-lang-book-ko
https://doc.rust-lang.org/cargo

<h1>Rust</h1>

컴파일
rstc main.rs
실행
./main

러스트 컨벤션

1. 탭x 스페이스 4개

<h1>Cargo</h1>

Cargo는 러스트의 빌드 시스템 및 패키지 매니저이다.

toml(Tom's Obvious, Minimal Language)은 Cargo의 환경설정 포맷이다.

러스트에서는 코드의 패키지를 크레이트(crate)라고 부른다.
단일 소스 코드 파일을 실행하고 전달하는 경우도 크레이트로 간주한다.

- 바이너리 크레이트
  cli나 서버와 같이 실행할 수 있는 실행 파일로 컴파일 할 수 있는 프로그램
  main에는 실행 파일이 실행될때 발생하는 일을 정의해야한다.
- 라이브러리 크레이트
  다른사람들이 자신의 프로젝트에 dependency로 추가할 수 있는 프로젝트 ex) rand
  일반적으로 라이브러리와 같은 의미이다.

crate root는 러스트 컴파일러가 시작하고 크레이트 루트 모듈을 구성하는 소스파일이다.
패키지는 일련의 기능을 제공하는 하나 이상의 크레이트 묶음이다. Cargo.toml 파일은 Node.js의 package.json과 비슷한 역할을 한다.
패키지에는 많은 바이너리 크레이트가 포함될 수 있지만, 라이브러리 크레이트는 하나만 포함 가능하다.
Cargo.toml에는 src/main.rs과 같은 실행파일에 대한 정보가 없는데, src/main.rs가 패키지와 이름이 같은 바이너리 크레이트의 크레이트 루트라는 규칙을 따른다.
라이브러리 크레이트의 경우 src/lib.rs가 크레이트 루트라는것을 알고 있다. Cargo는 빌드를 위해 크레이트 루트 파일을 rustc에 전달한다.

빌드 명령어

```bash
cargo build
```

target/debug에 프로젝트 이름으로 실행파일이 생성된다.
빌드를 처음 실행하면 최상위 디렉토리에 Cargo.lock 파일이 생성된다. node.js의 package-lock.json 같은것인듯

실행 명령어 - 빌드 명령어를 따로 입력하지 않아도 바로 빌드 - 실행 된다.
파일이 변경되지 않았다면 빌드하지않고 바로 실행파일을 실행한다.

```bash
cargo run .
```

컴파일 되는지 확인하는 명령어

```bash
cargo check
```

개발중에 컴파일이 되는지 안되는지 확인하려면 굳이 빌드를 하지 않아도 된다.
check가 build명령어보다 훨씬 빠르다.

릴리즈 빌드

```bash
cargo build --release
```

그냥 빌드보다 훨씬 오래걸리지만, 그냥 빌드보다 코드가 최적화되어 더 빠르다.

<br/>

현재 프로젝트에서 사용하고 있는 모든 패키지 문서를 모아서 보기

```bash
cargo doc --open
```

<h1>모듈</h1>

- 절대경로
  크레이트 루트에서 시작하는 전체경로
  외부 크레이트의 경우 절대경로는 크레이트 이름으로 시작하며, 현재 크레이트 코드의 경우 크레이트 리터럴로 시작한다.
- 상대경로
  현재 모듈에서 시작하며 self, super, 현재 모듈의 식별자를 사용한다.

절대경로, 상대경로 모두 :: 으로 구분된 하나 이상의 식별자로 끝난다.

문서주석
/// 슬래시 세개. 마크다운 지원

```bash
# target/doc에 HTML 문서 생성됨
$ cargo doc
$ cargo doc --open
```
