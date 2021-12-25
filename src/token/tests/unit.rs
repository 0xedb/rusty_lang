use crate::token::{literal, Unit};
use std::collections::HashMap;

#[test]
fn test_unit() {
    let mut keywords = HashMap::new();

    keywords.insert(literal::IF, Unit::If);
    keywords.insert(literal::ELSE, Unit::Else);
    keywords.insert(literal::LET, Unit::Let);
    keywords.insert(literal::FN, Unit::Fn);
    keywords.insert(literal::TRUE, Unit::True);
    keywords.insert(literal::FALSE, Unit::False);
    keywords.insert("name", Unit::Ident);
    keywords.insert(",", Unit::Ident);

    for (k, _) in keywords.iter() {
        let mut val = Unit::Illegal;

        if let Some(x) = keywords.get(*k) {
            val = *x
        }

        let ident = Unit::lookup_ident(k.to_string());
        assert!(val == ident);
    }
}
