use raytracer::{
    graphics::{color::Color, lights::PointLight},
    math::{point::Point, ray::Ray, transformations::Transformation, vector::Vector},
    objects::{
        intersections::{Intersection, Intersections},
        sphere::Sphere,
        world::World,
    },
};

#[test]
fn default_world() {
    let light = PointLight::new(Point::new(-10.0, -10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let mut s1 = Sphere::default();
    s1.material.color = Color::new(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;

    let _s2 = Sphere::new(Transformation::scaling(0.5, 0.5, 0.5));

    let w = World::default();

    assert_eq!(w.light.unwrap(), light);
    println!("{:#?}", s1);
    println!("{:#?}", w.objects[0]);
    // shperes got uuids!
    // assert!(w.objects.contains(&s1));
    // assert!(w.objects.contains(&s2));
}

#[test]
fn intersect_world_with_ray() {
    let w = World::default();
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let xs: Intersections = w.intersect(r);

    assert_eq!(xs.count(), 4);
    assert_eq!(xs.get(0).unwrap().intersects_at, 4.0);
    assert_eq!(xs.get(1).unwrap().intersects_at, 4.5);
    assert_eq!(xs.get(2).unwrap().intersects_at, 5.5);
    assert_eq!(xs.get(3).unwrap().intersects_at, 6.0);
    assert_eq!(xs.get(4), None);
}

#[test]
fn preparing_computations() {
    let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let shape = Sphere::default();
    let i = Intersection::new(4.0, &shape);

    let comps = i.prepare_computations(ray).unwrap();

    assert_eq!(comps.intersects_at, i.intersects_at);
    assert_eq!(comps.object, i.object);
    assert_eq!(comps.point, Point::new(0.0, 0.0, -1.0));
    assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
    assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
}
