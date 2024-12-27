use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum Direction {
    Bottom,
    Left,
    Top,
    Right,
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

impl Direction {
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

    #[must_use]
    pub fn to_index(&self) -> usize {
        self.to_number() - 1
    }

    #[must_use]
    pub fn from_number(num: usize) -> Result<Self, ParseDirectionError<usize>> {
        match num {
            5 => Ok(Self::TopLeft),
            1 => Ok(Self::Top),
            6 => Ok(Self::TopRight),
            2 => Ok(Self::Left),
            3 => Ok(Self::Right),
            7 => Ok(Self::BottomLeft),
            4 => Ok(Self::Bottom),
            8 => Ok(Self::BottomRight),
            _ => Err(ParseDirectionError::new(num)),
        }
    }
}

impl TryFrom<usize> for Direction {
    type Error = ParseDirectionError<usize>;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::from_number(value)
    }
}

impl TryFrom<char> for Direction {
    type Error = ParseDirectionError<char>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Self::Top),
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            'V' | 'v' => Ok(Self::Bottom),
            _ => Err(ParseDirectionError::new(value)),
        }
    }
}

pub struct ParseDirectionError<T: Copy> {
    pub val: T,
}

impl<T: Copy + Debug + Display> ParseDirectionError<T> {
    pub fn new(val: T) -> Self {
        Self { val }
    }
}

impl<T: Copy + Debug> Debug for ParseDirectionError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not parse {:?} into a direction", self.val)
    }
}

impl<T: Copy + Display> Display for ParseDirectionError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not parse {} into a direction", self.val)
    }
}

impl<T: std::marker::Copy + Debug + Display> Error for ParseDirectionError<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
