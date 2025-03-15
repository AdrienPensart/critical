use serde::Serialize;

use super::errors::CriticalErrorKind;

pub const RATINGS: &[f64] = &[0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0];

#[derive(Default, Hash, Serialize, Debug, Clone, Copy)]
pub enum Rating {
    #[default]
    Zero,
    Half,
    One,
    OneAndHalf,
    Two,
    TwoAndHalf,
    Three,
    ThreeAndHalf,
    Four,
    FourAndHalf,
    Five,
}

impl std::fmt::Display for Rating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rating: f64 = (*self).into();
        write!(f, "{}", rating)
    }
}

impl From<Rating> for f64 {
    fn from(rating: Rating) -> f64 {
        match rating {
            Rating::Zero => 0.0,
            Rating::Half => 0.5,
            Rating::One => 1.0,
            Rating::OneAndHalf => 1.5,
            Rating::Two => 2.0,
            Rating::TwoAndHalf => 2.5,
            Rating::Three => 3.0,
            Rating::ThreeAndHalf => 3.5,
            Rating::Four => 4.0,
            Rating::FourAndHalf => 4.5,
            Rating::Five => 5.0,
        }
    }
}

impl TryFrom<f64> for Rating {
    type Error = CriticalErrorKind;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        match value {
            0.0 => Ok(Rating::Zero),
            0.5 => Ok(Rating::Half),
            1.0 => Ok(Rating::One),
            1.5 => Ok(Rating::OneAndHalf),
            2.0 => Ok(Rating::Two),
            2.5 => Ok(Rating::TwoAndHalf),
            3.0 => Ok(Rating::Three),
            3.5 => Ok(Rating::ThreeAndHalf),
            4.0 => Ok(Rating::Four),
            4.5 => Ok(Rating::FourAndHalf),
            5.0 => Ok(Rating::Five),
            _ => Err(CriticalErrorKind::InvalidRating {
                path: "".to_string(),
                rating: value,
            }),
        }
    }
}
