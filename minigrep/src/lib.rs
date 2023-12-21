use std::env;
use std::error::Error;
use std::fs;

/// Config 설명
pub struct Config {
    /// 쿼리
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    /*
       https://doc.rust-lang.org/std/env/fn.args.html
       impl Iterator<Item = String>: 트레이트 바운드를 가진 제네릭 타입
       impl Trait 구문의 사용을 의미하며 args가 Iterator 타입을 구현하고 String 아이템을 반환하는 어떤 타입이든 될 수 있음을 나타낸다.
    */
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        /*
         IGNORE_CASE=0 cargo run -- to poem.txt
        */
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

/*
  search 함수로 반환되는 데이터는 search 함수로 전달된 contents 인자만큼 오래 유지될것라는 의미
  search 반환값은 query가 아니라 contents로부터 만들었기때문에 컴파일러가 query에서 만들었다고 착각하지 않게 함
*/
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /*
       iterator는 for loop과 성능이 거의 동등하다.
       즉, iterator는 고수준 추상화임에도 불구하고 별도의 런타임 오버헤드가 없다. (제로 오버헤드, 제로 코스트 추상화)
    */
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
