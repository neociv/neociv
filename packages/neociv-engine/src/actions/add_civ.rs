use rusqlite::{params, ToSql};

use crate::actions::EngineAction;

pub struct AddCiv<'action> {
    id: &'action str,
}

impl<'action> EngineAction<'_> for AddCiv<'_> {
    fn name(&self) -> &'static str {
        "add_civ"
    }

    fn sql(&self) -> &'static str {
        "INSERT INTO civs () VALUES (?1)"
    }

    fn params(&self) -> &[&dyn ToSql] {
        params![self.id]
    }

    fn props(&self) -> Vec<&'static str> {
        vec!["civs"]
    }
}
