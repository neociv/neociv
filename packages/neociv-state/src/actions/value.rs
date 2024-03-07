use crate::actions::common::{ActionTimer, TimedAction};

/// NOT "mod" but "supply" - that way it's easily targetted and from whom

pub struct ModValue {
    target: String,
}

impl TimedAction for ModValue {
    fn when(&self) -> ActionTimer {
        ActionTimer::Instant
    }
}
