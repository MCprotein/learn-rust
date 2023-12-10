use std::collections::HashMap;

pub fn hash_map() {
    /*
     HashMap은 일반적인 컬렉션중에 가장 덜 사용되서 prelude내에 자동으로 가져와지는 기능에 포함되어있지 않다.
     해시맵 생성 빌트인 매크로도 없다.
     벡터와 마찬가지로 해시맵은 힙에 저장된다.
     벡터와 마찬가지로 모든 키와 값은 각각 같은 타입이여야 한다.
    */
    let mut scores: HashMap<String, i32> = HashMap::new();

    /*
     String을 key로 사용할 경우, 소유권이 hashMap안으로 이동한다.
     값에 대한 참조를 키로 사용한다면 해시맵이 유효할때동안 그 값도 유효해야한다.
    */
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yello"), 10);
    // scores.insert("Yello", 10);
    let team_name = String::from("Blue");
    /*
     get 메소드는 Option<&V>를 반환한다.
      Option은 Some(&V) 또는 None을 가질 수 있다.
      Option(&V)는 키에 해당하는 값을 찾았다는 의미이다.
      None은 키에 해당하는 값이 없다는 의미이다.
      copied()는 Option<&T>를 Option<T>로 변환한다.
      unwrap_or은 None이면 default값을 반환하고, Option<T>이면 T를 반환한다.
    */
    let my_score = scores.get(&team_name).unwrap_or(&0);
    println!("my_score: {:?}", my_score);

    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    /*
     HashMap의 insert 메소드는 여러 번 호출시 새 값으로 덮어씌운다.
     entry 메소드를 통해 특정 키의 존재 여부를 알 수 있다. 반환값: enum Entry
     Entry에 대한 or_insert 메소드는 해당 키 값이 있으면 Entry가 가지고 있는 값을 반환하고, 없으면 default값을 insert 한다.
    */
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Grey")).or_insert(50);
    println!("{:?}", scores);

    /*
     키 값이 있으면 1 증가시키고, 없으면 0 삽입
    */
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        /*
         or_insert 메소드는 새로운 키를 위한 빈 값을 만들고, 그 빈 값을 참조하는 mutable 참조자를 반환한다.
         그래서 아래와 같이 *를 사용하여 값을 증가시킬 수 있다.
        */
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}
