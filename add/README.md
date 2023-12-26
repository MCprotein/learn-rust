node.js와 nestjs에 있는 workspace와 같다. (monorepo)

최상위 Cargo.toml에 아래와 같이 작성한다.

```toml
[workspace]
members = ["adder", "add_one"]
```

컴파일된 파일은 각각의 디렉토리에 있는게 아니라 최상위 target 폴더에 존재한다.

각각의 workspace는 독립적이기때문에 참조하려면 해당 디렉토리의 Cargo.toml에 다음과 같이 작성해야한다.

```toml
[dependencies]
add_one = { path = "../add_one" }
```

Cargo.lock은 최상위에만 생성된다.

외부 패키지 사용하려면 각 디렉토리의 Cargo.toml dependencies에 작성하면 되는데, 각각 작성하더라도 실제로 사용하는 버전은 똑같다.
버전 다르게쓰면 아래와 같은 에러가 발생한다.

```bash
error: failed to select a version for the requirement `rand = "^0.8.6"`
candidate versions found which didn't match: 0.8.5, 0.8.4, 0.8.3, ...
location searched: crates.io index
required by package `adder v0.1.0 (/Users/sys/Desktop/code/rust-hello-world/add/adder)`
if you are looking for the prerelease package it needs to be specified explicitly
    rand = { version = "0.7.0-pre.2" }
perhaps a crate was updated and forgotten to be re-vendored?
```
