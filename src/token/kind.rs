// contains token type

use super::unit::Unit;

pub struct Type {
    unit: Unit,
    literal: Option<String>,
}

impl Type {
    pub fn new(unit: Unit, literal: String) -> Self {
        Type {
            unit,
            literal: Some(literal),
        }
    }
}
