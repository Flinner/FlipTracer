use raytracer::math::{point::Point, vector::Vector};

#[derive(Debug)]
struct Projectile {
    position: Point,
    velocity: Vector,
}
struct Environment {
    gravity: Vector,
    wind: Vector,
}

pub fn main() {
    let init_projectile = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.0, 0.0).normalize(),
    };
    let environment = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut projectile: Projectile = init_projectile;

    while projectile.position.y >= 0.0 {
        println!("{:#?}", projectile.position);
        projectile = tick(&environment, &projectile);
    }
    println!("{:#?}", projectile.position);
}

fn tick(environment: &Environment, projectile: &Projectile) -> Projectile {
    Projectile {
        position: projectile.position.displacment(&projectile.velocity),
        velocity: projectile.velocity + environment.gravity + environment.wind,
    }
}
