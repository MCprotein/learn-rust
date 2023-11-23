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

러스트에서는 코드의 패키지를 크레이트(create)라고 부른다.

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
