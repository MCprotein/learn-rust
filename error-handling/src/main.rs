use std::{
    error::Error,
    fs::File,
    io::{self, ErrorKind, Read},
};

fn main() {
    let result = read_file_box();
    println!("{:?}", result);
}

fn read_file() {
    let greeting_file_result = File::open("hello.txt");

    /*
       match
    */
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // Err(error) => panic!("Problem opening the file: {:?}", error),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file_info) => file_info,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    println!("{:?}", greeting_file);

    /*
       if
    */

    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    /*
       unwrap은 에러 발생 시 panic!을 호출
       expect는 에러 발생시 panic!을 호출하며 메시지를 지정할 수 있다.
    */
    let greeting_file = File::open("hello.txt").unwrap();
    let greeting_file = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

/*
    에러처리할때 ?를 사용하면 match나 조건문을 사용하지 않아도 된다.
    ?는 match 표현식과 동일한 방식으로 동작하도록 정의되어 있기 때문에, Result나 Option을 반환하는 함수에서만 사용할 수 있다.
    ?는 에러가 발생하면 즉시 해당 함수를 호출한 함수에게 에러를 반환한다.
    ?는 에러가 발생하지 않으면 Ok값을 반환한다.
*/
fn read_username_from_file_question_mark() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_question_mark_short() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

/*
    Result<Box<dyn Error>>: 모든 종류의 에러를 의미
*/
fn read_file_box() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
