pub enum ImprovementType {
    Building,
}

pub trait Improvement {
    fn improvement_type() -> ImprovementType;
    fn improvement_integrity() -> f32;
    fn improvement_content() -> String;
}