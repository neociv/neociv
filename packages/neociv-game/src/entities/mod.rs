pub mod materials;

pub trait NeocivEntity<T> {
    fn id(&self) -> &str;
    fn content(&self) -> &T;
}
