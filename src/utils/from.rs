#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub enum From {
    Bottom,
    Left,
    Top,
    Right,
}

impl From {
    #[must_use]
    pub fn to_usize(&self) -> usize {
        match self {
            From::Bottom => 0,
            From::Left => 1,
            From::Top => 2,
            From::Right => 3,
        }
    }
}
