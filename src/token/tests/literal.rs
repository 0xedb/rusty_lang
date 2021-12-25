use crate::token::literal::*;
use std::collections::HashMap;

#[test]
fn test_literals() {
    let mut mp_char = HashMap::new();
    mp_char.insert(DOT, '.');
    mp_char.insert(COLON, ':');
    mp_char.insert(LPAREN, '(');
    mp_char.insert(RPAREN, ')');
    mp_char.insert(LCURL, '{');
    mp_char.insert(RCURL, '}');
    mp_char.insert(LBRAC, '[');
    mp_char.insert(RBRAC, ']');
    mp_char.insert(PLUS, '+');
    mp_char.insert(MINUS, '-');
    mp_char.insert(ASSIGN, '=');
    mp_char.insert(BANG, '!');
    mp_char.insert(STAR, '*');
    mp_char.insert(SLASH, '/');
    mp_char.insert(GT, '>');
    mp_char.insert(LT, '<');
    mp_char.insert(EOF, '\0');

    for (k, v) in mp_char.iter() {
        assert!(k == v)
    }

    let mut mp_str = HashMap::new();

    mp_str.insert(IF, "se");
    mp_str.insert(ELSE, "ana");
    mp_str.insert(LET, "ma");
    mp_str.insert(FN, "ye");
    mp_str.insert(EQUAL, "==");
    mp_str.insert(TRUE, "nukre");
    mp_str.insert(FALSE, "ntro");
    mp_str.insert(NOTEQUAL, "!=");

    for (k, v) in mp_str.iter() {
        assert!(k == v)
    }
}
