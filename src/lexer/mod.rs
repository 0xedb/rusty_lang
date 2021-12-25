use super::token::{self, literal, Type, Unit};

#[derive(Debug)]
pub struct Lexer {
    character: Option<char>,
    input: String,
    read_pos: usize,
    current_pos: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input,
            character: Option::None,
            read_pos: 0,
            current_pos: 0,
        };

        l.read_char();
        l
    }

    fn read_char(&mut self) {
        println!("data \t:{:?}", self);
        if self.current_pos >= self.input.len() {
            self.character = None;
        } else {
            self.character = self.input.chars().nth(self.read_pos);
        }
        self.current_pos = self.read_pos;
        self.read_pos += 1;
    }

    pub fn next_token(&mut self) -> token::Type {
        let mut tok = Type::new(Unit::Illegal, String::from(""));
        println!("selffffff {:?}", self);
        self.eat_whitespace();

        println!("CHAR-> {:?}", self.character);

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
            Some(literal::EOF) => tok = Type::new(Unit::Illegal, literal::EOF.to_string()),
            Some(literal::ASSIGN) => {
                if self.next_char_is(literal::ASSIGN) {
                    tok = Type::new(Unit::Equal, literal::EQUAL.to_string())
                } else {
                    tok = Type::new(Unit::Assign, literal::ASSIGN.to_string())
                }
            }
            Some(literal::BANG) => {
                if self.next_char_is(literal::ASSIGN) {
                    tok = Type::new(Unit::NotEqual, literal::NOTEQUAL.to_string());
                } else {
                    tok = Type::new(Unit::Bang, literal::BANG.to_string())
                }
            }
            Some(ch) => {
                println!("checking....");
                if self.is_letter(ch) {
                    let literal = self.read_ident();
                    tok = Type::new(Unit::lookup_ident(literal.clone()), literal)
                } else if self.is_digit(ch) {
                    tok = Type::new(Unit::Int, self.read_dig())
                }
            }

            None => (),
        };

        println!("almost");
        self.read_char();

        println!("here::{:?}", tok);

        return tok;
    }

    fn next_char_is(&self, ch: char) -> bool {
        if self.read_pos < self.input.len() - 1 {
            if let Some(x) = self.input.chars().nth(self.read_pos + 1) {
                return x == ch;
            };
        }

        false
    }

    fn eat_whitespace(&mut self) {

        let is_space = |character| {
            if let Some(ch) = character {
                return ch == '\n' || ch == '\r' || ch == '\t' || ch == ' ';
            }

            false
        };

        while is_space(self.character) {
            self.read_char();
        }
    }

    fn is_letter(&self, ch: char) -> bool {
        ch == '_' || (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z')
    }

    fn is_digit(&self, ch: char) -> bool {
        ch >= '0' && ch <= '9'
    }

    fn read_ident(&mut self) -> String {
        let pos = self.current_pos;

        if let Some(ch) = self.character {
            while self.is_letter(ch) {
                self.read_char();
            }
        }

        self.input[pos..self.current_pos].to_string()
    }

    fn read_dig(&mut self) -> String {
        let pos = self.current_pos;

        if let Some(ch) = self.character {
            println!("diggging....");
            while self.is_digit(ch) {
                println!("on it");
                self.read_char();
            }
        }

        println!("done digging...");
        self.input[pos..self.current_pos].to_string()
    }
}
