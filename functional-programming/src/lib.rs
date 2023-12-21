#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    let v1_rest: Vec<i32> = v1_iter.clone().map(|&x| x).collect();
    assert_eq!(v1_rest, vec![2, 3]);
    assert_eq!(v1_iter.next(), Some(&2));

    assert_eq!(v1_iter.next(), Some(&3));

    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    /*
       sum 메소드가 호출된 후에는 v1_iter를 사용할 수 없다.
       sum이 v1_iter의 소유권을 가져가기 때문이다.
    */
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

#[test]
fn iterator_collect() {
    let v1: Vec<i32> = vec![1, 2, 3];
    /*
       collect 메소드가 이터레이터를 소비하기때문에 collect메소드가 호출되기 전까지는 이터레이터 항목을 처리하지 않는다.
        이런 특성을 lazy 하다고 한다.
    */
    let v2 = v1.iter().map(|x| x + 1);
    let v2: Vec<_> = v2.collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    /*
       iter(): collection의 소유권을 가져가지 않고 불변참조 생성
       이터레이션 중에도 원래 collection은 변경, 소모 없이 사용 가능
       into_iter(): collection의 소유권을 가져가는 이터레이터 생성
       원래 collection 사용 불가
    */
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);
        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
