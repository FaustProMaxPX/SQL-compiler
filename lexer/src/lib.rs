pub mod token {
    #[derive(Debug, PartialEq)]
    pub enum Keyword {
        Select,
        Create,
        Update,
        Delete,
        From,
        Where,
        Ident(String),
        String(String),
        Number(i32),
        Greater,
        Less,
        GreaterEq,
        LessEq,
        Equal,
        NotEqual,
        And,
        Or,
        AllLine,
        In,
        LeftBracket,
        RightBracket,
        Table,
    }
}

pub mod lexer {

    use super::token::Keyword;

    pub fn parse(input: &str) -> Option<Vec<Keyword>> {
        let mut tokens = Vec::new();
        let mut pos = 0;
        let mut counter = 0;
        while pos < input.len() {
            let c = input.get_char(pos)?;
            // println!("{}", c);
            if c.is_whitespace() {
                pos += 1;
                continue;
            }

            if c.is_digit(10) {
                let start = pos;
                pos += 1;
                while pos < input.len() && input.get_char(pos)?.is_digit(10) {
                    pos += 1;
                }
                tokens.push(Keyword::Number(input[start..pos].parse().unwrap()));
                continue;
            }

            match c {
                '=' if input.get_char(pos + 1)? == '=' => {
                    pos += 2;
                    tokens.push(Keyword::Equal);
                }
                '>' => {
                    if input.get_char(pos + 1)? == '=' {
                        pos += 2;
                        tokens.push(Keyword::GreaterEq);
                    } else {
                        pos += 1;
                        tokens.push(Keyword::Greater);
                    }
                }
                '<' => {
                    if input.get_char(pos + 1)? == '=' {
                        pos += 2;
                        tokens.push(Keyword::LessEq);
                    } else {
                        pos += 1;
                        tokens.push(Keyword::Less);
                    }
                }
                '!' if input.get_char(pos + 1)? == '=' => {
                    pos += 2;
                    tokens.push(Keyword::NotEqual);
                }
                '"' | '\'' => {
                    let start = pos + 1;
                    pos = start + input[start..].find(c).unwrap();
                    tokens.push(Keyword::String(input[start..pos].to_string()));
                    pos += 1;
                }
                '*' => {
                    pos += 1;
                    tokens.push(Keyword::AllLine);
                }
                '(' => {
                    counter += 1;
                    pos += 1;
                    tokens.push(Keyword::LeftBracket);
                }
                ')' => {
                    if counter <= 0 {
                        eprintln!("sql出现词法错误");
                        return None;
                    }
                    pos += 1;
                    tokens.push(Keyword::RightBracket);
                }
                _ if c.is_alphabetic() => {
                    let start = pos;
                    pos += 1;
                    while pos < input.len() && input.get_char(pos)?.is_alphanumeric() {
                        pos += 1;
                    }
                    let ident = &input[start..pos];
                    
                    match ident.to_uppercase().as_str() {
                        "SELECT" => tokens.push(Keyword::Select),
                        "DELETE" => tokens.push(Keyword::Delete),
                        "CREATE" => tokens.push(Keyword::Create),
                        "UPDATE" => tokens.push(Keyword::Update),
                        "FROM" => tokens.push(Keyword::From),
                        "WHERE" => tokens.push(Keyword::Where),
                        "AND" => tokens.push(Keyword::And),
                        "OR" => tokens.push(Keyword::Or),
                        "IN" => tokens.push(Keyword::In),
                        "TABLE" => tokens.push(Keyword::Table),
                        _ => tokens.push(Keyword::Ident(ident.to_string())),
                    }
                }
                _ => {
                    eprintln!("无法识别的数据流");
                    return None;
                },
            }
        }

        Some(tokens)
    }

    trait Index {
        fn get_char(&self, index: usize) -> Option<char>;
    }

    impl Index for &str {
        fn get_char(&self, index: usize) -> Option<char> {
            utf8_slice::slice(self, index, index + 1).chars().next()
        }
    }
}
