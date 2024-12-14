#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum To {
    Bottom,
    Left,
    Top,
    Right,
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

impl To {
    #[must_use]
    pub fn move_to(&self, (row, col): (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Self::Bottom => Some((row + 1, col)),
            Self::Left => {
                if col == 0 {
                    return None;
                }
                Some((row, col - 1))
            }
            Self::Top => {
                if row == 0 {
                    return None;
                }
                Some((row - 1, col))
            }
            Self::Right => Some((row, col + 1)),
            Self::TopRight => {
                if row == 0 {
                    return None;
                }
                Some((row - 1, col + 1))
            }
            Self::TopLeft => {
                if row == 0 || col == 0 {
                    return None;
                }
                Some((row - 1, col - 1))
            }
            Self::BottomRight => Some((row + 1, col + 1)),
            Self::BottomLeft => {
                if col == 0 {
                    return None;
                }
                Some((row + 1, col - 1))
            }
        }
    }

    #[must_use]
    pub fn all_directions() -> [Self; 8] {
        [
            Self::TopLeft,
            Self::Top,
            Self::TopRight,
            Self::Left,
            Self::Right,
            Self::BottomLeft,
            Self::Bottom,
            Self::BottomRight,
        ]
    }

    #[must_use]
    pub fn cardinal_directions() -> [Self; 4] {
        [Self::Top, Self::Left, Self::Right, Self::Bottom]
    }

    #[must_use]
    pub fn x_directions() -> [Self; 4] {
        [
            Self::TopLeft,
            Self::TopRight,
            Self::BottomLeft,
            Self::BottomRight,
        ]
    }

    #[must_use]
    pub fn turn_right_90(&self) -> Self {
        match self {
            Self::TopLeft => Self::TopRight,
            Self::Top => Self::Right,
            Self::TopRight => Self::BottomRight,
            Self::Left => Self::Top,
            Self::Right => Self::Bottom,
            Self::BottomLeft => Self::TopLeft,
            Self::Bottom => Self::Left,
            Self::BottomRight => Self::BottomLeft,
        }
    }
    #[must_use]
    pub fn turn_left_90(&self) -> Self {
        match self {
            Self::TopRight => Self::TopLeft,
            Self::Right => Self::Top,
            Self::BottomRight => Self::TopRight,
            Self::Top => Self::Left,
            Self::Bottom => Self::Right,
            Self::TopLeft => Self::BottomLeft,
            Self::Left => Self::Bottom,
            Self::BottomLeft => Self::BottomRight,
        }
    }

    #[must_use]
    pub fn turn_180(&self) -> Self {
        match self {
            Self::TopRight => Self::BottomLeft,
            Self::Right => Self::Left,
            Self::BottomRight => Self::TopLeft,
            Self::Top => Self::Bottom,
            Self::Bottom => Self::Top,
            Self::TopLeft => Self::BottomRight,
            Self::Left => Self::Right,
            Self::BottomLeft => Self::TopRight,
        }
    }

    #[must_use]
    pub fn to_number(&self) -> usize {
        match self {
            Self::TopLeft => 5,
            Self::Top => 1,
            Self::TopRight => 6,
            Self::Left => 2,
            Self::Right => 3,
            Self::BottomLeft => 7,
            Self::Bottom => 4,
            Self::BottomRight => 8,
        }
    }
}
