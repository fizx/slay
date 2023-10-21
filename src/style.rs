use crate::types::{Align, Direction, FixedScalar, Scalar};

pub struct Style {
    pub width: Option<Scalar>,
    pub height: Option<Scalar>,
    pub min_width: Option<Scalar>,
    pub max_width: Option<Scalar>,
    pub min_height: Option<Scalar>,
    pub max_height: Option<Scalar>,
    pub padding: Option<FixedScalar>,
    pub gap: Option<FixedScalar>,
    pub align: Option<Align>,
    pub anchor: Option<Align>,
    pub direction: Direction,
}

impl Style {
    // Constructor
    pub fn new() -> Self {
        Self {
            width: None,
            height: None,
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
            padding: None,
            gap: None,
            align: None,
            anchor: None,
            direction: Direction::Y,
        }
    }

    // Convenience methods for each field
    pub fn width(mut self, width: Scalar) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: Scalar) -> Self {
        self.height = Some(height);
        self
    }

    pub fn min_width(mut self, min_width: Scalar) -> Self {
        self.min_width = Some(min_width);
        self
    }

    pub fn max_width(mut self, max_width: Scalar) -> Self {
        self.max_width = Some(max_width);
        self
    }

    pub fn min_height(mut self, min_height: Scalar) -> Self {
        self.min_height = Some(min_height);
        self
    }

    pub fn max_height(mut self, max_height: Scalar) -> Self {
        self.max_height = Some(max_height);
        self
    }

    pub fn padding(mut self, padding: FixedScalar) -> Self {
        self.padding = Some(padding);
        self
    }

    pub fn gap(mut self, gap: FixedScalar) -> Self {
        self.gap = Some(gap);
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = Some(align);
        self
    }

    pub fn anchor(mut self, anchor: Align) -> Self {
        self.anchor = Some(anchor);
        self
    }
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }
}
