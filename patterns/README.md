패턴은 두 가지 형태가 있다.
- 가변적인 형태 (refutable)
- 무조건적인 형태 (irrefutable)

아래 코드에서 x는 어떤 값이든 일치하므로 일치하지 않을 수 없다.
따라서 이것은 irrefutable 패턴이다.
```rust
let x = 5
```

아래코드는 실패할 수 있으므로 refutable 패턴이다.
```rust
if let Some(x) = a_value {}
```

함수 parameter, let, for loop은 irrefutable 패턴만 허용한다.
if let, while let은 refutable, irrefutable 모두 허용하지만, refutable는 경고로 나타난다.

```rust
/*
만약 some_option_value가 None이라면 패턴 Some(x)와 일치하지 않아 실패하므로 refutable 패턴이다.
하지만 let은 None을 사용할 수 있는 유효한 동작이 없으므로 irrefutable 패턴만 허용한다.
 */
let Some(x) = some_option_value;
```
위와 같은 코드는 아래와 같이 수정하여 우회할 수 있다.
```rust
if let Some(x) = some_option_value {
    println!("{}", x);
}
```
아래와 같은 코드는 항상 오류가 없으므로 컴파일 경고가 발생한다.
```rust
if let x = 5 {
    println!("{}", x);
};
```