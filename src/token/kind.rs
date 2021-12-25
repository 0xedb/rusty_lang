use super::unit::Unit;

#[derive(Debug)]
pub struct Type {
    pub unit: Unit,
    pub literal: Option<String>,
}

impl Type {
    pub fn new(unit: Unit, literal: String) -> Self {
        Type {
            unit,
            literal: Some(literal),
        }
    }
}
