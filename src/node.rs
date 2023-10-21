use crate::context;
use crate::style::Style;
use crate::types::{Align, Direction, FixedScalar, Rectangle, Scalar, Size};

pub struct Node {
    pub children: Vec<Node>,
    pub style: Style,
    pub computed: Rectangle,
    min_size: Size,
    dirty: bool,
    measure: Option<Box<dyn Fn() -> Size>>,
}

impl Node {
    pub fn compute_layout(&mut self, context: &context::Context) {
        if self.dirty {
            self.computed = self.compute_self();
            self.dirty = false;
        }
    }
    pub fn minimum_size(&mut self, context: &context::Context) {
        if self.min_size == Size::zero() {
            self.min_size = self.compute_size(context.as_minimum());
        }
    }

    pub fn compute_self(&mut self) -> Rectangle {
        Rectangle::zero()
    }

    pub fn compute_size(&mut self, context: context::Context) -> Size {
        if let Some(measure) = &self.measure {
            self.min_size = measure();
            return self.min_size;
        }

        let mut min_size = Size::zero();

        let child_context = context.child_context(self);

        for child in &mut self.children {
            child.minimum_size(&child_context);
            min_size = min_size.add(child.min_size, self.style.direction);
        }

        min_size.width += self.style.padding.unwrap_or(0) * 2;
        min_size.height += self.style.padding.unwrap_or(0) * 2;

        let gaps = std::cmp::max(self.children.len() as i32 - 1, 0);

        match self.style.direction {
            Direction::X => {
                min_size.width += self.style.gap.unwrap_or(0) * gaps;
            }
            Direction::Y => {
                min_size.height += self.style.gap.unwrap_or(0) * gaps;
            }
            _ => {}
        }

        if self.style.width.is_some() {
            min_size.width = context.pixelize_width(self.style.width.unwrap());
        }
        if self.style.max_width.is_some() {
            min_size.width = min_size
                .width
                .min(context.pixelize_width(self.style.max_width.unwrap()));
        }
        if self.style.min_width.is_some() {
            min_size.width = min_size
                .width
                .max(context.pixelize_width(self.style.min_width.unwrap()));
        }

        if self.style.height.is_some() {
            min_size.height = context.pixelize_height(self.style.height.unwrap());
        }
        if self.style.max_height.is_some() {
            min_size.height = min_size
                .height
                .min(context.pixelize_height(self.style.max_height.unwrap()));
        }
        if self.style.min_height.is_some() {
            min_size.height = min_size
                .height
                .max(context.pixelize_height(self.style.min_height.unwrap()));
        }

        min_size
    }
}

impl Node {
    // Tree manipulation methods
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: Style::new(),
            computed: Rectangle::zero(),
            min_size: Size::zero(),
            dirty: true,
            measure: None,
        }
    }

    pub fn add_child(mut self, child: Node) -> Self {
        self.children.push(child);
        self.dirty = true;
        self
    }

    // Remove a child from the node at a specific index
    pub fn remove_child(&mut self, index: usize) {
        if index < self.children.len() {
            self.children.remove(index);
            self.dirty = true; // Mark the node as dirty after removing a child
        }
    }

    // Get a specific child (immutable reference)
    pub fn get_child(&self, index: usize) -> Option<&Node> {
        self.children.get(index)
    }

    // Get a specific child (mutable reference)
    pub fn get_child_mut(&mut self, index: usize) -> Option<&mut Node> {
        self.children.get_mut(index)
    }

    // Update the style of the node
    pub fn set_style(&mut self, style: Style) -> &mut Self {
        self.style = style;
        self.dirty = true; // Mark the node as dirty after updating the style
        self
    }

    // Set the measure function
    pub fn set_measure(&mut self, measure: Box<dyn Fn() -> Size>) -> &mut Self {
        self.measure = Some(measure);
        self.dirty = true; // Mark the node as dirty after setting the measure function
        self
    }

    // Forcefully mark the node (and potentially its children) as dirty
    pub fn mark_dirty(&mut self, deep: bool) -> &mut Self {
        self.dirty = true;
        if deep {
            for child in &mut self.children {
                child.mark_dirty(true);
            }
        }
        self
    }
}

impl Node {
    // Delegated builder methods for Style properties

    pub fn width(mut self, width: Scalar) -> Self {
        self.style = self.style.width(width);
        self.dirty = true;
        self
    }

    pub fn height(mut self, height: Scalar) -> Self {
        self.style = self.style.height(height);
        self.dirty = true;
        self
    }

    pub fn min_width(mut self, min_width: Scalar) -> Self {
        self.style = self.style.min_width(min_width);
        self.dirty = true;
        self
    }

    pub fn max_width(mut self, max_width: Scalar) -> Self {
        self.style = self.style.max_width(max_width);
        self.dirty = true;
        self
    }

    pub fn min_height(mut self, min_height: Scalar) -> Self {
        self.style = self.style.min_height(min_height);
        self.dirty = true;
        self
    }

    pub fn max_height(mut self, max_height: Scalar) -> Self {
        self.style = self.style.max_height(max_height);
        self.dirty = true;
        self
    }

    pub fn padding(mut self, padding: FixedScalar) -> Self {
        self.style = self.style.padding(padding);
        self.dirty = true;
        self
    }

    pub fn gap(mut self, gap: FixedScalar) -> Self {
        self.style = self.style.gap(gap);
        self.dirty = true;
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.style = self.style.align(align);
        self.dirty = true;
        self
    }

    pub fn anchor(mut self, anchor: Align) -> Self {
        self.style = self.style.anchor(anchor);
        self.dirty = true;
        self
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.style = self.style.direction(direction);
        self.dirty = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::context::Context;
    use crate::fixtures::*;

    #[test]
    fn test_minimum_size() {
        let mut node = nested_layout_fixture();

        let size = node.compute_size(Context::new(1000, 1000).as_minimum());

        // vstack(150x(50+50), 200x100 == 200x200)

        assert_eq!(size.width, 200);
        assert_eq!(size.height, 200);
    }

    #[test]
    fn test_natural_size() {
        let mut node = vertical_stretch_layout_fixture();

        let size = node.compute_size(Context::new(1000, 1000));

        // vstack(150x(50+50), 200x100 == 200x200)

        assert_eq!(size.width, 200);
        assert_eq!(size.height, 1000);
    }

    // Add more tests as needed
}
