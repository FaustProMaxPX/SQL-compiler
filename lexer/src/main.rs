use std::{env, process, io, fs};

use sql_lexer::lexer::parse;

fn read(mut iter: impl ExactSizeIterator<Item = String>) -> Result<String, &'static str> {
    if iter.len() <= 1 {
        let mut s = String::new();
        println!("Please input your sql: ");
        io::stdin().read_line(&mut s).unwrap();
        return Ok(s);
    } else {
        iter.next();
        match iter.next() {
            Some(s) => Ok(fs::read_to_string(s).unwrap()),
            None => return Err("not enough arguments"),
        }
    }
}

fn main() {
    let arg = read(env::args()).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });
    // let v = parse("SELECT * FROM stu WHERE id < '100' AND name = '好'");
    let v = parse(&arg);
    println!("{:#?}", v);
}

#[cfg(test)]
mod lexer_test {

    use sql_lexer::{lexer::*, token::Keyword};

    #[test]
    
    fn case_insensitive_test() {
        let v = parse("select *\n from a in (select * from a)");
        match v {
            Some(v) => assert_eq!(vec![
                Keyword::Select,
                Keyword::AllLine,
                Keyword::From,
                Keyword::Ident("a".to_string()),
                Keyword::In,
                Keyword::LeftBracket,
                Keyword::Select,
                Keyword::AllLine,
                Keyword::From,
                Keyword::Ident("a".to_string()),
                Keyword::RightBracket
            ], v),
            None => assert!(true)
        }
    }
}

