pub fn iter() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    println!("{v1_iter:?}");

    for val in v1_iter {
        println!("Got: {}", val);
    }
}
