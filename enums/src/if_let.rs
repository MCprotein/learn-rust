/*
  if와 else가 러스트에서는 if let과 else이다.
*/
pub fn main() {
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three")
    } else {
        println!("not three")
    }
}
