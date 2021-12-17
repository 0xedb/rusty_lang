// use token::{Type, Unit, literal};
use crate::token::{self, literal, Type, Unit};

#[derive(Debug)]
pub struct Lexer {
    character: Option<char>,
    input: String,
    read_pos: usize,
    current_pos: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            character: Option::None,
            read_pos: 0,
            current_pos: 0,
        }
    }

    fn read_char(&mut self) {
        if self.current_pos >= self.input.len() {
            self.character = Some('0');
        } else {
            self.character = self.input.chars().nth(self.read_pos)
        }
        self.current_pos = self.read_pos;
        self.read_pos += 1;
    }

    fn next_token(&mut self) -> token::Type {
        let mut tok: token::Type;

        match self.character {
            Some(literal::DOT) => tok = Type::new(Unit::Dot, literal::DOT.to_string()),
            Some(literal::COLON) => tok = Type::new(Unit::Colon, literal::COLON.to_string()),
            Some(literal::COMMA) => tok = Type::new(Unit::Comma, literal::COMMA.to_string()),
            Some(literal::SEMICOL) => tok = Type::new(Unit::Semicol, literal::SEMICOL.to_string()),
            Some(literal::LPAREN) => tok = Type::new(Unit::Lparen, literal::LPAREN.to_string()),
            Some(literal::RPAREN) => tok = Type::new(Unit::Rparen, literal::RPAREN.to_string()),
            Some(literal::LCURL) => tok = Type::new(Unit::Lcurl, literal::LCURL.to_string()),
            Some(literal::RCURL) => tok = Type::new(Unit::Rcurl, literal::RCURL.to_string()),
            Some(literal::RBRAC) => tok = Type::new(Unit::Rbrac, literal::RBRAC.to_string()),
            Some(literal::LBRAC) => tok = Type::new(Unit::Lbrac, literal::LBRAC.to_string()),
            Some(literal::PLUS) => tok = Type::new(Unit::Plus, literal::PLUS.to_string()),
            Some(literal::MINUS) => tok = Type::new(Unit::Minus, literal::MINUS.to_string()),
            Some(literal::STAR) => tok = Type::new(Unit::Star, literal::STAR.to_string()),
            Some(literal::SLASH) => tok = Type::new(Unit::Slash, literal::SLASH.to_string()),
            Some(literal::GT) => tok = Type::new(Unit::Gt, literal::GT.to_string()),
            Some(literal::LT) => tok = Type::new(Unit::Lt, literal::LT.to_string()),
            
            // handle equal
            Some(literal::ASSIGN) => tok = Type::new(Unit::Assign, literal::ASSIGN.to_string()),
            // handle notequal
            Some(literal::BANG) => tok = Type::new(Unit::Bang, literal::BANG.to_string()),

            _ => tok = Type::new(Unit::Illegal, String::from("")),
        };

        self.read_char();
        return tok;
    }

    fn eat_whitespace(&self) {
        while let Some(ch) = self.character {}
    }
}
