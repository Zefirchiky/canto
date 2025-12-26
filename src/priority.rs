#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Priority {
    Highest = 5,
    HighMid = 4,
    Mid = 3,
    LowMid = 2,
    Lowest = 1,
}

impl Priority {
    pub const fn list() -> [Priority; 5] {
        [
            Self::Highest,
            Self::HighMid,
            Self::Mid,
            Self::LowMid,
            Self::Lowest,
        ]
    }
}
