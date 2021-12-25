use crate::token::{kind::Type, literal, unit::Unit};
use std::collections::HashMap;

fn kind_type_eq(s: &String, u: &Unit, m: &HashMap<String, Unit>) -> bool {
    let k = Type::new(*u, s.clone());

    k.unit == *u && k.literal == Some(s.clone()) && m.get(s) == Some(u)
}

#[test]
fn test_kind() {
    let mut mp = HashMap::new();

    mp.insert(literal::COLON.to_string(), Unit::Colon);
    mp.insert(literal::BANG.to_string(), Unit::Bang);
    mp.insert(literal::ASSIGN.to_string(), Unit::Assign);
    for (k, v) in mp.iter() {
        assert!(kind_type_eq(k, v, &mp))
    }
}
