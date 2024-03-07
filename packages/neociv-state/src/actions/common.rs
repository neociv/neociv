pub enum ActionTimer {
    Instant,
    StartOfTurn,
    EndOfTurn,
    Delayed(u32),
}

pub trait TimedAction {
    fn when(&self) -> ActionTimer;
}
