fn main() {
    /*
       스칼라: 하나의 값으로 표현되는 타입
       총 네가지 스칼라 타입: 정수형, 부동소수점 숫자, boolean, 문자
    */

    /*
        정수형
        Length	Signed	Unsigned
        8-bit	i8	    u8
        16-bit	i16	    u16
        32-bit	i32	    u32
        64-bit	i64	    u64
        arch	isize	usize

        n: 비트 수
        signed: -2^(n-1) ~ 2^(n-1)-1
        unsigned: 0 ~ 2^n-1

        정수형 리터럴
        Decimal	        98_222
        Hex	            0xff
        Octal	        0o77
        Binary	        0b1111_0000
        Byte (u8 only)	b'A'

        일반적으로는 i32가 가장빠르다.
    */

    /*
       부동소수점
       f32, f64
       기본타입: f64 -> 최신 CPU에서는 f64가 f32와 비슷한 속도를 보이며 더 정밀한 표현이 가능하기 때문
    */
    let _x = 2.0;
    let _y: f32 = 3.0;

    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _remainder = 43 % 5;

    let _t = true;
    let _f = false;

    /*
     char는 가장 근본적인 알파벳 타입이다.
     string은 ""를 사용하지만, char는 ''를 사용한다.
     ASCII보다 더 많은 값을 표현할 수 있다.
    */
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    /*
        복합 타입
    */

    /*
       튜플
       node.js의 구조분해할당이 된다.
       요소의 타입이 각각 달라도 된다.
    */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = tup;
    let _five_hndred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;

    /*
       배열
       요소의 타입이 모두 같아야 한다.
       고정된 길이를 갖는다.
       데이터가 heap보다는 stack에 할당되기를 원하는 경우에 사용한다.
       벡터타입: 유사 집합체, 확장과 축소가 가능하다.
       stack에 단일 메모리 뭉치로 할당된다.
       인덱스를 사용할 수 있다.
    */
    let a = [1, 2, 3, 4, 5];
    let _frist = a[0];
    let _second = a[1];
    /*
       length보다 큰 인덱스를 사용하면 컴파일타임에서는 에러가 발생하지 않지만,
       런타임에서 에러가 발생하여 panic이 발생한다.
       index out of bounds: the length is 5 but the index is 8
    */
    println!("The value of frist is: {}", a[8]);
}
