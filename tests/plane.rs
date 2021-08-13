use raytracer::{
    math::{
        point::Point,
        ray::Ray,
        vector::{self, Vector},
    },
    objects::shape::{self, Shape},
};

#[test]
fn normal_of_planes() {
    let tests = vec![
        (
            shape::default::plane().normal_at(Point::new(1.0, 0.0, 0.0)),
            vector::UNIT_Y,
        ),
        (
            shape::default::plane().normal_at(Point::new(0.0, 1.0, 0.0)),
            vector::UNIT_Y,
        ),
        (
            shape::default::plane().normal_at(Point::new(1.0, 0.0, 1.0)),
            vector::UNIT_Y,
        ),
    ];

    for test in tests {
        assert_eq!(test.0.unwrap(), test.1);

        // normals are normalized
        assert_eq!(test.1, test.1.normalize())
    }
}

#[test]
#[should_panic]
fn intersecets_with_ray_parallel_to_plane() {
    let origin = Point::new(0.0, 10.0, 0.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);

    let s: Shape = shape::default::plane();
    let _xs = s.intersects(&ray).unwrap();
}

#[test]
#[should_panic]
// infinity intersections, but plane is infinittly thin. no intersections
fn intersecets_with_a_coplanar_ray() {
    let origin = Point::new(0.0, 0.0, 0.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);

    let s: Shape = shape::default::plane();
    let _xs = s.intersects(&ray).unwrap();
}

#[test]
fn intersecets_with_a_plane_from_above() {
    let origin = Point::new(0.0, 1.0, 0.0);
    let direction = Vector::new(0.0, -1.0, 0.0);

    let ray = Ray::new(origin, direction);

    let s: Shape = shape::default::plane();
    let xs = s.intersects(&ray).unwrap();
    assert_eq!(xs.count(), 1);
    assert_eq!(xs.get_intersection(0).unwrap(), 1.0);
    assert_eq!(xs.get_intersection(1), None);
    assert_eq!(xs.get_object(0).unwrap(), s)
}

#[test]
fn intersecets_with_a_plane_from_below() {
    let origin = Point::new(0.0, -1.0, 0.0);
    let direction = Vector::new(0.0, 1.0, 0.0);

    let ray = Ray::new(origin, direction);

    let s: Shape = shape::default::plane();
    let xs = s.intersects(&ray).unwrap();
    assert_eq!(xs.count(), 1);
    assert_eq!(xs.get_intersection(0).unwrap(), 1.0);
    assert_eq!(xs.get_intersection(1), None);
    assert_eq!(xs.get_object(0).unwrap(), s)
}
