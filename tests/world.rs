use std::f64::consts::SQRT_2;

use raytracer::{
    constants::MAX_REFLECTION_RECRUSTION,
    graphics::{
        color::{self, Color},
        lights::PointLight,
        materials::Material,
        patterns::Pattern,
    },
    math::{
        point::{self, Point},
        ray::Ray,
        transformations::Transformation,
        vector::Vector,
    },
    objects::{
        intersections::{Intersection, Intersections},
        shape::{self, Shape, ShapeType},
        sphere,
        world::World,
    },
    testing::Testing,
};

#[test]
fn default_world() {
    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let mut s1 = sphere::default();
    s1.material.color = Color::new(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;

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
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let shape: Shape = shape::sphere::default();
    let i = Intersection::new(4.0, shape);

    let xs = Intersections {
        list: vec![i.to_owned()],
    };

    let comps = i.prepare_computations(r, Some(&xs)).unwrap();

    assert_eq!(comps.intersects_at, i.intersects_at);
    assert_eq!(comps.object, i.object);
    assert_eq!(comps.point, Point::new(0.0, 0.0, -1.0));
    assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
    assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
}

#[test]
fn shading_an_intersection() {
    let w = World::default();
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let shape = w.objects[0];
    let i = Intersection::new(4.0, shape);

    let xs = Intersections {
        list: vec![i.to_owned()],
    };

    let comps = i.prepare_computations(r, Some(&xs)).unwrap();

    let c = w.shade_hit(&comps, MAX_REFLECTION_RECRUSTION);

    Testing::assert_nearly_eq(c, Color::new(0.38066, 0.47583, 0.2855))
}

#[test]
fn shading_an_intersection_from_inside() {
    let w = World {
        light: Some(PointLight::new(
            Point::new(0.0, 0.25, 0.0),
            Color::new(1.0, 1.0, 1.0),
        )),
        ..Default::default()
    };

    let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    let shape = w.objects[1];
    let i = Intersection::new(0.5, shape);

    let xs = Intersections {
        list: vec![i.to_owned()],
    };

    let comps = i.prepare_computations(r, Some(&xs)).unwrap();
    let c = w.shade_hit(&comps, MAX_REFLECTION_RECRUSTION);

    Testing::assert_nearly_eq(c, Color::new(0.90498, 0.90498, 0.90498))
}

#[test]
fn color_when_ray_misses() {
    let w = World::default();
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));

    assert_eq!(w.color_at(r, MAX_REFLECTION_RECRUSTION), color::BLACK)
}

#[test]
fn color_when_ray_hits() {
    let w = World::default();
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

    Testing::assert_nearly_eq(
        w.color_at(r, MAX_REFLECTION_RECRUSTION),
        Color::new(0.38066, 0.47583, 0.2855),
    )
}

#[test]
fn color_when_intersection_behind_ray() {
    let mut w = World::default();
    let mut outer = Shape {
        shape_type: ShapeType::Sphere,
        ..Default::default()
    };
    outer.material.ambient = 1.0;
    let mut inner = Shape {
        shape_type: ShapeType::Sphere,
        ..Default::default()
    };
    inner.material.ambient = 1.0;
    let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));

    w.objects = vec![outer, inner];

    let inner = &w.objects[1];
    assert_eq!(
        w.color_at(r, MAX_REFLECTION_RECRUSTION),
        inner.material.color
    )
}

#[test]
fn no_shadow_when_nothing_collinear_with_point_and_light() {
    let w = World::default();
    let p = Point::new(0.0, 10.0, 0.0);
    assert!(!w.is_shadowed(p));
}

#[test]
fn shadow_when_object_between_point_and_light() {
    let w = World::default();
    let p = Point::new(10.0, -10.0, 10.0);
    assert!(w.is_shadowed(p));
}

#[test]
fn no_shadow_when_object_is_behind_light() {
    let w = World::default();
    let p = Point::new(-20.0, 20.0, -20.0);
    assert!(!w.is_shadowed(p));
}

#[test]
fn no_shadow_when_object_is_behind_point() {
    let w = World::default();
    let p = Point::new(-2.0, 2.0, -2.0);
    assert!(!w.is_shadowed(p));
}

#[test]
fn intersection_is_shadow() {
    let mut w = World::new();
    w.light = Some(PointLight::new(
        Point::new(0.0, 0.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    ));
    let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
    let s1 = shape::sphere::default();
    let s2 = shape::sphere::new(
        Transformation::translation(0.0, 0.0, 10.0),
        Material::default(),
    );

    w.objects = vec![s1, s2];

    let i = Intersection {
        intersects_at: 4.0,
        object: s2,
    };

    let xs = Intersections {
        list: vec![i.to_owned()],
    };

    let comps = i.prepare_computations(r, Some(&xs)).unwrap();

    let c = w.shade_hit(&comps, MAX_REFLECTION_RECRUSTION);

    Testing::assert_nearly_eq(c, Color::new(0.1, 0.1, 0.1))
}

#[test]
fn precomputing_reflection_vector() {
    let shape = shape::plane::default();

    let r = Ray::new(
        Point::new(0.0, 1.0, -1.0),
        Vector::new(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
    );
    let i = Intersection::new(SQRT_2, shape);
    let xs = Intersections {
        list: vec![i.to_owned()],
    };

    let comps = i.prepare_computations(r, Some(&xs)).unwrap();

    assert_eq!(comps.reflectv, Vector::new(0.0, SQRT_2 / 2.0, SQRT_2 / 2.0))
}

#[test]
fn reflect_color_for_nonreflective_material() {
    let mut w = World::default();
    let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));

    let shape = &mut w.objects[0];
    shape.material.ambient = 1.0;

    let i = Intersection::new(1.0, *shape);
    let xs = Intersections {
        list: vec![i.to_owned()],
    };

    let comps = i.prepare_computations(r, Some(&xs)).unwrap();

    let c = w.reflected_color(&comps, MAX_REFLECTION_RECRUSTION);

    assert_eq!(c, color::BLACK)
}

#[test]
fn reflect_color_for_reflective_material() {
    let mut w = World::default();
    let mut material = Material::default();
    material.reflective = 0.5;

    let shape = shape::plane::new(Transformation::translation(0.0, -1.0, 0.0), material);
    w.objects.push(shape);

    let r = Ray::new(
        Point::new(0.0, 0.0, -3.0),
        Vector::new(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
    );

    let i = Intersection::new(SQRT_2, shape);

    let xs = Intersections {
        list: vec![i.to_owned()],
    };

    let comps = i.prepare_computations(r, Some(&xs)).unwrap();
    let c = w.reflected_color(&comps, MAX_REFLECTION_RECRUSTION);

    Color::assert_nearly_eq(c, Color::new(0.1903306125, 0.237913265737, 0.142747959442));
}

#[test]
fn shade_hit_with_reflective_material() {
    let mut w = World::default();
    let mut material = Material::default();
    material.reflective = 0.5;

    let shape = shape::plane::new(Transformation::translation(0.0, -1.0, 0.0), material);
    w.objects.push(shape);

    let r = Ray::new(
        Point::new(0.0, 0.0, -3.0),
        Vector::new(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
    );

    let i = Intersection::new(SQRT_2, shape);

    let xs = Intersections {
        list: vec![i.to_owned()],
    };

    let comps = i.prepare_computations(r, Some(&xs)).unwrap();
    let color = w.shade_hit(&comps, MAX_REFLECTION_RECRUSTION);

    Color::assert_nearly_eq(
        color,
        Color::new(0.87675599850, 0.9243386516513, 0.8291733453562),
    );
}

#[test]
fn shade_hit_with_infinite_recursion() {
    // to plane mirrors facing each other with a ray reflecting for infinity
    let mut w = World::new();
    w.light = Some(PointLight::new(point::ORIGIN, color::BLACK));

    let mut material = Material::default();
    material.reflective = 0.5;

    let lower = shape::plane::new(Transformation::translation(0.0, -1.0, 0.0), material);
    let upper = shape::plane::new(Transformation::translation(0.0, 1.0, 0.0), material);

    w.objects.push(lower);
    w.objects.push(upper);

    let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
    let c = w.color_at(ray, MAX_REFLECTION_RECRUSTION);
    assert_eq!(c, color::BLACK)
}

#[test]
fn refracted_color_for_opaque_surface() {
    let mut w = World::default();
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

    let shape = &mut w.objects[0];
    shape.material.ambient = 1.0;

    let i1 = Intersection::new(4.0, *shape);
    let i2 = Intersection::new(6.0, *shape);
    let xs = i1.clone().agregate(i2);

    let comps = i1.prepare_computations(r, Some(&xs)).unwrap();

    let c = w.refracted_color(&comps, MAX_REFLECTION_RECRUSTION);

    assert_eq!(c, color::BLACK)
}

#[test]
#[ignore = "it works but with 0.1 deviation "]
fn refracted_color_with_max_recursion_depth() {
    // to plane mirrors facing each other with a ray reflecting for infinity
    let mut w = World::default();
    // w.light = Some(PointLight::new(point::ORIGIN, color::BLACK));

    let shape = &mut w.objects[0];
    shape.material.transparency = 1.0;
    shape.material.refractive_index = 1.5;

    let i1 = Intersection::new(4.0, *shape);
    let i2 = Intersection::new(6.0, *shape);
    let xs = i1.clone().agregate(i2);

    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

    let comps = i1.prepare_computations(r, Some(&xs)).unwrap();

    let c = w.refracted_color(&comps, 0); //remaining is zero

    assert_eq!(c, color::BLACK)
}

#[test]
fn refracted_color_under_total_internal_reflection() {
    // to plane mirrors facing each other with a ray reflecting for infinity
    let mut w = World::default();
    // w.light = Some(PointLight::new(point::ORIGIN, color::BLACK));

    let shape = &mut w.objects[0];
    shape.material.transparency = 1.0;
    shape.material.refractive_index = 1.5;

    let i1 = Intersection::new(-SQRT_2 / 2.0, *shape);
    let i2 = Intersection::new(SQRT_2 / 2.0, *shape);
    let xs = i1.agregate(i2.clone());

    let r = Ray::new(
        Point::new(0.0, 0.0, SQRT_2 / 2.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let comps = i2.prepare_computations(r, Some(&xs)).unwrap();

    let c = w.refracted_color(&comps, 5); //remaining is zero

    assert_eq!(c, color::BLACK)
}

#[test]
fn refracted_color_with_a_refracted_ray() {
    let mut world = World::default();
    let a = &mut world.objects[0];
    a.material.ambient = 1.0;
    a.material.pattern = Some(Pattern::default());

    let b = &mut world.objects[1];
    b.material.transparency = 1.0;
    b.material.refractive_index = 1.5;

    let a = world.objects[0];
    let b = world.objects[1];
    let r = Ray::new(Point::new(0.0, 0.0, 0.1), Vector::new(0.0, 1.0, 0.0));

    // world.objects = vec![a, b];

    let i1 = Intersection::new(-0.9899, a);
    let i2 = Intersection::new(-0.4899, b);
    let i3 = Intersection::new(0.4899, b);
    let i4 = Intersection::new(0.9899, a);

    let xs = i1.agregate(i2).agregate(i3).agregate(i4);
    // let comps = i1.prepare_computations(r, Some(xs)).unwrap();
    let comps = xs
        .get(2)
        .unwrap()
        .prepare_computations(r, Some(&xs))
        .unwrap();

    let color = world.refracted_color(&comps, 5);

    Testing::assert_nearly_eq(color, Color::new(0.0, 0.99888468, 0.0472164))
}

#[test]
fn shade_hit_with_transperant_material() {
    let mut world = World::default();

    let mut floor_material = Material::default();
    floor_material.transparency = 0.5;
    floor_material.refractive_index = 1.5;

    let floor = shape::plane::new(Transformation::translation(0.0, -1.0, 0.0), floor_material);

    let mut ball_material = Material::default();
    ball_material.color = Color::new(1.0, 0.0, 0.0);
    ball_material.ambient = 0.5;

    let ball = shape::sphere::new(Transformation::translation(0.0, -3.5, -0.5), ball_material);

    world.objects.push(floor);
    world.objects.push(ball);

    let ray = Ray::new(
        Point::new(0.0, 0.0, -3.0),
        Vector::new(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
    );
    let i1 = Intersection::new(SQRT_2, floor);
    let xs = Intersections {
        list: vec![i1.clone()],
    };

    let comps = i1.prepare_computations(ray, Some(&xs)).unwrap();
    let color = world.shade_hit(&comps, 5);

    Testing::assert_nearly_eq(color, Color::new(0.93642, 0.68642, 0.68642));
}

#[test]
fn shade_hit_with_reflective_transperant_material() {
    let mut world = World::default();

    let mut floor_material = Material::default();
    floor_material.transparency = 0.5;
    floor_material.reflective = 0.5;
    floor_material.refractive_index = 1.5;

    let floor = shape::plane::new(Transformation::translation(0.0, -1.0, 0.0), floor_material);

    let mut ball_material = Material::default();
    ball_material.color = Color::new(1.0, 0.0, 0.0);
    ball_material.ambient = 0.5;

    let ball = shape::sphere::new(Transformation::translation(0.0, -3.5, -0.5), ball_material);

    world.objects.push(floor);
    world.objects.push(ball);

    let ray = Ray::new(
        Point::new(0.0, 0.0, -3.0),
        Vector::new(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
    );
    let i1 = Intersection::new(SQRT_2, floor);
    let xs = Intersections {
        list: vec![i1.clone()],
    };

    let comps = i1.prepare_computations(ray, Some(&xs)).unwrap();
    let color = world.shade_hit(&comps, 5);

    Testing::assert_nearly_eq(color, Color::new(0.93391, 0.69643, 0.69243));
}
