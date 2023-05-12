use rusqlite::ToSql;

pub mod add_civ;

pub trait EngineAction<'action> {
    fn name(&self) -> &'static str;
    fn sql(&self) -> &'action str;
    fn params(&self) -> &[&dyn ToSql];
    fn props(&self) -> Vec<&'action str>;
}
