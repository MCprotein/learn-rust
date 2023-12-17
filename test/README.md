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
