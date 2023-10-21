use crate::{
    node::Node,
    types::{FixedScalar, Scalar},
};

#[derive(Clone)]
pub struct Context {
    pub(crate) root_width: FixedScalar,
    pub(crate) root_height: FixedScalar,
    pub(crate) sized_ancestor_width: FixedScalar,
    pub(crate) sized_ancestor_height: FixedScalar,
    pub(crate) desired_width: FixedScalar,
    pub(crate) desired_height: FixedScalar,
}

impl Context {
    pub fn new(root_width: FixedScalar, root_height: FixedScalar) -> Self {
        Self {
            root_width,
            root_height,
            sized_ancestor_width: root_width,
            sized_ancestor_height: root_height,
            desired_width: root_width,
            desired_height: root_height,
        }
    }

    pub fn as_minimum(&self) -> Self {
        Self {
            desired_width: 0,
            desired_height: 0,
            ..self.clone()
        }
    }

    pub fn child_context(&self, node: &Node) -> Self {
        let sized_ancestor_width = match node.style.width {
            Some(width) => self.pixelize_width(width),
            None => self.sized_ancestor_width,
        };

        let sized_ancestor_height = match node.style.height {
            Some(height) => self.pixelize_height(height),
            None => self.sized_ancestor_height,
        };

        Self {
            root_width: self.root_width,
            root_height: self.root_height,
            sized_ancestor_width,
            sized_ancestor_height,
            desired_width: node.computed.width,
            desired_height: node.computed.height,
        }
    }

    pub fn pixelize_width(&self, value: Scalar) -> FixedScalar {
        match value {
            (value, crate::types::ScalarUnit::Px) => value,
            (value, crate::types::ScalarUnit::Percent) => {
                let percent = value as f32 / 100.0;
                let width = self.sized_ancestor_width as f32;
                (percent * width) as i32
            }
            (value, crate::types::ScalarUnit::Vw) => {
                let percent = value as f32 / 100.0;
                let width = self.root_width as f32;
                (percent * width) as i32
            }
            (value, crate::types::ScalarUnit::Vh) => {
                let percent = value as f32 / 100.0;
                let height = self.root_height as f32;
                (percent * height) as i32
            }
        }
    }
    pub fn pixelize_height(&self, value: Scalar) -> FixedScalar {
        match value {
            (value, crate::types::ScalarUnit::Px) => value,
            (value, crate::types::ScalarUnit::Percent) => {
                let percent = value as f32 / 100.0;
                let height = self.sized_ancestor_height as f32; // Use ancestor height
                (percent * height) as i32
            }
            (value, crate::types::ScalarUnit::Vw) => {
                let percent = value as f32 / 100.0;
                let width = self.root_width as f32;
                (percent * width) as i32 // Vw still uses width
            }
            (value, crate::types::ScalarUnit::Vh) => {
                let percent = value as f32 / 100.0;
                let height = self.root_height as f32; // Use root height
                (percent * height) as i32
            }
        }
    }
}
