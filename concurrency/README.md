Thread
메인 스레드가 종료되면 생성되었던 다른 스레드들도 모두 종료된다.

Do not communicate by sharing memory; instead, share memory by communicating. - [Go 공식문서](https://golang.org/doc/effective_go.html#concurrency)
Rust 표준 라이브러리가 Channel 구현을 제공한다.

러스트가 동시성을 처리하는 방식은 대부분 언어의 일부가 아니라 크레이트로 구현된다. 따라서 언어보다 크레이트 업데이트가 더 빨리되기때문에 공식문서 검색을 해봐야한다.
