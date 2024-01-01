```text
Object-oriented programs are made up of objects. 
An object packages both data and the procedures that operate on that data. 
The procedures are typically called methods or operations.

- Design Patterns: Elements of Reusable Object-Oriented Software
```

이 정의를 기준으로 하면 Rust는 객체 지향 언어이다.
구조체와 열거형은 데이터를 가지고 있고, impl 블록은 구조체와 열거형에 대한 메소드를 제공한다.
구조체와 열거형을 객체라고 부르지는 않지만, 객체 정의와 같은 기능을 제공한다.

러스트에는 상속이 없지만 trait 메소드 구현을 통해 비슷하게 구현할 수 있다.
최근 상속이 좋은 선택이 아닌 경우가 많아졌다. 자식 클래스가 항상 부모 클래스의 모든 특성을 공유할 필요가 없음에도 불구하고 이런 특성을 공유하기 때문이다.
그래서 상속 대신 트레이트 객체를 사용하는 다른 방식을 취한다.