테스트할때 기본적으로 병렬로 실행된다.
이것을 막기 위해서는 --test-threads 플래그를 붙인다.

```bash
$ cargo test -- --test-threads=1
```

사용할 수 있는 명령어는 아래 명령어로 알 수 있다.
둘이 다른 명령어이다.

```bash
$ cargo test --help
$ cargo test -- --help
```

성공한 테스트는 stdout 메세지가 안보이고 실패한 테스트에서만 보이는데, 아래 플래그를 사용하여 성공한 테스트도 보이게 할 수 있다.

```bash
$ cargo test -- --show-output
$ cargo test -- --nocapture
```

`#[cfg(test)]`는 `cargo test` 를 실행할때만 컴파일&실행 하라는 의미이다.
따라서 일반적인 `cargo build` 때에는 테스트가 컴파일되지 않는다. 통합테스트는 다른 디렉토리에 있기때문에 해당 어노테이션이 필요하지 않다.
cfg는 configuration을 의미하며 인자로 받는 환경에서만 컴파일 & 실행 되도록 한다.

아래 코드에서 internal_adder는 private이지만 use super::\*로 가져올 수 있다.

```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

통합테스트는 tests 폴더를 만들어서 작성한다.
tests폴더안에 테스트 파일을 만들고, 그 안의 테스트코드에는 cfg(test)를 쓰지 않아도 된다. tests 폴더는 기본적으로 테스트 모듈로 취급되기때문이다.

특정 통합테스트만 실행

```bash
$ cargo test --test integration_test
```
