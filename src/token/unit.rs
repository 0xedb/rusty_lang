use super::literal;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Unit {
    EOF,     // eof
    Illegal, // illegal

    // identifiers + literals
    Int,   // integer
    Ident, // identifier: do_work, etc

    // delimiters
    Dot,     // .
    Colon,   // :
    Comma,   // ,
    Semicol, // ;

    Lparen, // (
    Rparen, // )
    Lcurl,  // {
    Rcurl,  // }
    Lbrac,  // [
    Rbrac,  //]

    // operators
    Plus,     // +
    Minus,    // -
    Equal,    // ==
    Assign,   // =
    NotEqual, // `!=
    Bang,     // `!
    Star,     // `*
    Slash,    // `/

    Gt, // >
    Lt, // <

    // keywords
    If,    // if
    Else,  // else
    Let,   // let
    Fn,    // fn
    True,  // true
    False, // false
}

impl Unit {
    pub fn lookup_ident(s: String) -> Self {
        let mut keywords = HashMap::new();

        keywords.insert(literal::IF.to_string(), Unit::If);
        keywords.insert(literal::ELSE.to_string(), Unit::Else);
        keywords.insert(literal::LET.to_string(), Unit::Let);
        keywords.insert(literal::FN.to_string(), Unit::Fn);
        keywords.insert(literal::TRUE.to_string(), Unit::True);
        keywords.insert(literal::FALSE.to_string(), Unit::False);

        if let Some(x) = keywords.get(&s) {
            return *x;
        }

        Unit::Ident
    }
}
