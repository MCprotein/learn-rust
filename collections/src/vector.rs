pub fn vector() {
    /* 벡터는 제네릭을 사용하여 구현된다. */
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    println!("{:?}", v);

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    /*
       아래 상황에서 컴파일 에러가 발생하는 이유
       벡터는 메모리에서 값을 서로 옆에 배치하기때문에,
       벡터 끝에 새로운 값을 추가하려면 새 메모리를 할당하고 이전 요소들을 새로운 공간에 복사해야할수도있다.
       이 경우 인덱스 0의 값을 참조하는 포인터는 할당 해제된 메모리를 가리킨다.

        let mut v = vec![1, 2, 3, 4, 5];
        let first = &v[0];
        v.push(6);
        println!("The first element is: {}", first);
    */

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    /*
       loop안에서 벡터에 값을 삽입하거나 제거하려고 하면 위와 같이 컴파일 에러가 발생한다.
    */
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);

    /*
       벡터는 동일한 타입의 값만 저장할 수 있다.
       하지만 열거형을 사용하면 다른 타입을 저장할 수 있다.
    */
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
}
