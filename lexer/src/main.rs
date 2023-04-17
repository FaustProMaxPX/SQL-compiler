use std::{env, process};

use sql_lexer::lexer::{read, parse};


fn main() {
    let arg = read(env::args()).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });
    // let v = parse("SELECT * FROM stu WHERE id < '100' AND name = '好'");
    let v = parse(&arg);
    println!("{:#?}", v);
}
