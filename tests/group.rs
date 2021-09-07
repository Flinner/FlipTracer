use raytracer::{
    math::transformations::Transformation,
    objects::{groups::Group, shape::Shape},
};

#[test]
fn creating_new_group() {
    let group = Group::default();
    assert_eq!(group.transform, Transformation::identity());
    assert_eq!(group.children.len(), 0)
}

#[test]
fn shape_has_parent_attr() {
    let shape = Shape::default();
    assert_eq!(shape.parent, None)
}
