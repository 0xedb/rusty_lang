// contains token units

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

    KwStart,
    If,    // if
    Else,  // else
    Let,   // let
    Fn,    // fn
    True,  // true
    False, // false
    KwEnd,
}
