use super::shape::Shape;
use crate::math::transformations::Transformation;

/// Represents a group of `Shape`s
/// `Shape`s link to parent `Group`s
/// resulting in a *bidirectional tree structure*
#[derive(PartialEq, Debug, Clone)]
pub struct Group<'a> {
    pub transform: Transformation,
    pub children: Vec<&'a Shape<'a>>,
}

impl<'a> Group<'a> {
    /// returns a new empty group with `Transformation::identity`
    pub fn new() -> Self {
        Group {
            transform: Transformation::identity(),
            children: vec![],
        }
    }

    /// Adds `Shape` to `Group`
    pub fn add(&'a mut self, shape: &'a mut Shape<'a>) {
        shape.parent = Some(self);
        // self.children.push(shape);
    }
}

impl Default for Group<'_> {
    fn default() -> Self {
        Self::new()
    }
}
