use std::fmt;

pub enum TokenType {
    Integer,
    Plus,
    EndOfFile,
}

struct Token {
    token_type: TokenType,
    value: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

fn main() {
    let t = Token {
        token_type: TokenType::Integer,
        value: "1".to_string(),
    };

    println!("{}", t);
}
