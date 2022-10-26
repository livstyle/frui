use frui::prelude::{Offset, Size};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::TextDirection;

pub trait AlignmentGeometry {
    fn resolve(&self, text_direction: &TextDirection) -> Alignment;
}
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Alignment {
    x: f64,
    y: f64,
}

impl AlignmentGeometry for Alignment {
    fn resolve(&self, _: &TextDirection) -> Alignment {
        *self
    }
}

impl Alignment {
    pub fn along<T: Into<Size>>(&self, other: T) -> Offset {
        let size: Size = other.into();
        let center_x = size.width / 2.0;
        let center_y = size.height / 2.0;
        Offset {
            x: center_x + self.x * center_x,
            y: center_y + self.y * center_y,
        }
    }

    pub const TOP_LEFT: Alignment = Alignment { x: -1.0, y: -1.0 };
    pub const TOP_CENTER: Alignment = Alignment { x: 0.0, y: -1.0 };
    pub const TOP_RIGHT: Alignment = Alignment { x: 1.0, y: -1.0 };
    pub const CENTER_LEFT: Alignment = Alignment { x: -1.0, y: 0.0 };
    pub const CENTER: Alignment = Alignment { x: 0.0, y: 0.0 };
    pub const CENTER_RIGHT: Alignment = Alignment { x: 1.0, y: 0.0 };
    pub const BOTTOM_LEFT: Alignment = Alignment { x: -1.0, y: 1.0 };
    pub const BOTTOM_CENTER: Alignment = Alignment { x: 0.0, y: 1.0 };
    pub const BOTTOM_RIGHT: Alignment = Alignment { x: 1.0, y: 1.0 };

    const PRELUDES: [(&Alignment, &'static str); 9] = [
        (&Alignment::TOP_LEFT, "Alignment::TOP_LEFT"),
        (&Alignment::TOP_CENTER, "Alignment::TOP_CENTER"),
        (&Alignment::TOP_RIGHT, "Alignment::TOP_RIGHT"),
        (&Alignment::CENTER_LEFT, "Alignment::CENTER_LEFT"),
        (&Alignment::CENTER, "Alignment::CENTER"),
        (&Alignment::CENTER_RIGHT, "Alignment::CENTER_RIGHT"),
        (&Alignment::BOTTOM_LEFT, "Alignment::BOTTOM_LEFT"),
        (&Alignment::BOTTOM_CENTER, "Alignment::BOTTOM_CENTER"),
        (&Alignment::BOTTOM_RIGHT, "Alignment::BOTTOM_RIGHT"),
    ];
}

impl Add for Alignment {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Alignment {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Alignment {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Alignment {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Neg for Alignment {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Alignment {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Display for Alignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (alignment, name) in Alignment::PRELUDES {
            if alignment == self {
                return write!(f, "{}", name);
            }
        }
        write!(f, "Alignment({}, {})", &self.x, &self.y)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AlignmentDirectional {
    start: f64,
    y: f64,
}

impl AlignmentGeometry for AlignmentDirectional {
    fn resolve(&self, text_direction: &TextDirection) -> Alignment {
        let start = match text_direction {
            TextDirection::Ltr => self.start,
            TextDirection::Rtl => -self.start,
        };
        Alignment {
            x: start,
            y: self.y,
        }
    }
}

impl AlignmentDirectional {
    const fn new(start: f64, y: f64) -> Self {
        Self { start, y }
    }

    pub const TOP_START: AlignmentDirectional = Self::new(-1., -1.);
    pub const TOP_CENTER: AlignmentDirectional = Self::new(0., -1.);
    pub const TOP_END: AlignmentDirectional = Self::new(1., -1.);
    pub const CENTER_START: AlignmentDirectional = Self::new(-1., 0.);
    pub const CENTER: AlignmentDirectional = Self::new(0., 0.);
    pub const CENTER_END: AlignmentDirectional = Self::new(1., 0.);
    pub const BOTTOM_START: AlignmentDirectional = Self::new(-1., 1.);
    pub const BOTTOM_CENTER: AlignmentDirectional = Self::new(0., 1.);
    pub const BOTTOM_END: AlignmentDirectional = Self::new(1., 1.);

    const PRELUDES: [(&AlignmentDirectional, &'static str); 9] = [
        (
            &AlignmentDirectional::TOP_START,
            "AlignmentDirectional::TOP_START",
        ),
        (
            &AlignmentDirectional::TOP_CENTER,
            "AlignmentDirectional::TOP_CENTER",
        ),
        (
            &AlignmentDirectional::TOP_END,
            "AlignmentDirectional::TOP_END",
        ),
        (
            &AlignmentDirectional::CENTER_START,
            "AlignmentDirectional::CENTER_START",
        ),
        (
            &AlignmentDirectional::CENTER,
            "AlignmentDirectional::CENTER",
        ),
        (
            &AlignmentDirectional::CENTER_END,
            "AlignmentDirectional::CENTER_END",
        ),
        (
            &AlignmentDirectional::BOTTOM_START,
            "AlignmentDirectional::BOTTOM_START",
        ),
        (
            &AlignmentDirectional::BOTTOM_CENTER,
            "AlignmentDirectional::BOTTOM_CENTER",
        ),
        (
            &AlignmentDirectional::BOTTOM_END,
            "AlignmentDirectional::BOTTOM_END",
        ),
    ];
}

impl Add for AlignmentDirectional {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        AlignmentDirectional {
            start: self.start + rhs.start,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for AlignmentDirectional {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        AlignmentDirectional {
            start: self.start - rhs.start,
            y: self.y - rhs.y,
        }
    }
}

impl Neg for AlignmentDirectional {
    type Output = Self;

    fn neg(self) -> Self::Output {
        AlignmentDirectional {
            start: -self.start,
            y: -self.y,
        }
    }
}

impl Mul<f64> for AlignmentDirectional {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        AlignmentDirectional::new(self.start * rhs, self.y * rhs)
    }
}

impl Div<f64> for AlignmentDirectional {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        AlignmentDirectional::new(self.start / rhs, self.y / rhs)
    }
}

impl Display for AlignmentDirectional {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (alignment, name) in AlignmentDirectional::PRELUDES {
            if alignment == self {
                return write!(f, "{}", name);
            }
        }
        write!(f, "AlignmentDirectional({}, {})", &self.start, &self.y)
    }
}
