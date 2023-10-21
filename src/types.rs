#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ScalarUnit {
    Px,
    Vh,
    Vw,
    Percent,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    X,
    Y,
    Z,
}

pub type Scalar = (i32, ScalarUnit);
pub type FixedScalar = i32;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum HAlign {
    Start,
    Center,
    End,
    Wide,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VAlign {
    Top,
    Center,
    Bottom,
    Tall,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Align {
    HAlign(HAlign),
    VAlign(VAlign),
    Both(HAlign, VAlign),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Rectangle {
    pub x: FixedScalar,
    pub y: FixedScalar,
    pub width: FixedScalar,
    pub height: FixedScalar,
}

impl Rectangle {
    pub fn zero() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }
    pub fn new(x: FixedScalar, y: FixedScalar, width: FixedScalar, height: FixedScalar) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Size {
    pub width: FixedScalar,
    pub height: FixedScalar,
}

impl Size {
    pub fn zero() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }

    pub fn new(width: FixedScalar, height: FixedScalar) -> Self {
        Self { width, height }
    }

    pub fn add(self, other: Self, direction: Direction) -> Self {
        match direction {
            Direction::X => Self {
                width: self.width + other.width,
                height: self.height.max(other.height),
            },
            Direction::Y => Self {
                width: self.width.max(other.width),
                height: self.height + other.height,
            },
            Direction::Z => Self {
                width: self.width.max(other.width),
                height: self.height.max(other.height),
            },
        }
    }
}
