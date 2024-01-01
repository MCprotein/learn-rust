Thread
메인 스레드가 종료되면 생성되었던 다른 스레드들도 모두 종료된다.

Do not communicate by sharing memory; instead, share memory by communicating. - [Go 공식문서](https://golang.org/doc/effective_go.html#concurrency)
Rust 표준 라이브러리가 Channel 구현을 제공한다.