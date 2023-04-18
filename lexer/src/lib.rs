pub mod token {
    #[derive(Debug, PartialEq)]
    pub enum Token {
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
    }
}

pub mod lexer {

    use super::token::Token;

    pub fn read(mut iter: impl Iterator<Item = String>) -> Result<String, &'static str> {
        iter.next();
        match iter.next() {
            Some(s) => Ok(s),
            None => return Err("not enough arguments"),
        }
    }

    pub fn parse(input: &str) -> Option<Vec<Token>> {
        let mut token = Vec::new();
        let mut pos = 0;
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
                token.push(Token::Number(input[start..pos].parse().unwrap()));
                continue;
            }

            match c {
                '=' if input.get_char(pos + 1)? == '=' => {
                    pos += 2;
                    token.push(Token::Equal);
                }
                '>' => {
                    if input.get_char(pos + 1)? == '=' {
                        pos += 2;
                        token.push(Token::GreaterEq);
                    } else {
                        pos += 1;
                        token.push(Token::Greater);
                    }
                }
                '<' => {
                    if input.get_char(pos + 1)? == '=' {
                        pos += 2;
                        token.push(Token::LessEq);
                    } else {
                        pos += 1;
                        token.push(Token::Less);
                    }
                }
                '!' if input.get_char(pos + 1)? == '=' => {
                    pos += 2;
                    token.push(Token::NotEqual);
                }
                '"' | '\'' => {
                    let start = pos + 1;
                    pos = start + input[start..].find(c).unwrap();
                    token.push(Token::String(input[start..pos].to_string()));
                    pos += 1;
                }
                '*' => {
                    pos += 1;
                    token.push(Token::AllLine);
                }
                _ if c.is_alphabetic() => {
                    let start = pos;
                    pos += 1;
                    while pos < input.len() && input.get_char(pos)?.is_alphanumeric() {
                        pos += 1;
                    }
                    let ident = &input[start..pos];
                    
                    match ident.to_uppercase().as_str() {
                        "SELECT" => token.push(Token::Select),
                        "DELETE" => token.push(Token::Delete),
                        "CREATE" => token.push(Token::Create),
                        "UPDATE" => token.push(Token::Update),
                        "FROM" => token.push(Token::From),
                        "WHERE" => token.push(Token::Where),
                        "AND" => token.push(Token::And),
                        "OR" => token.push(Token::Or),
                        _ => token.push(Token::Ident(ident.to_string())),
                    }
                }
                _ => pos += 1,
            }
        }

        Some(token)
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
