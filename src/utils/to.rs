#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
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

    pub fn cardinal_directions() -> [Self; 4] {
        [Self::Top, Self::Left, Self::Right, Self::Bottom]
    }

    pub fn x_directions() -> [Self; 4] {
        [
            Self::TopLeft,
            Self::TopRight,
            Self::BottomLeft,
            Self::BottomRight,
        ]
    }
}
