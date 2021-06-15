use raytracer::features::{point::Point, vector::Vector};
use raytracer::graphics::canvas::Canvas;
use raytracer::graphics::color;
use raytracer::graphics::ppm;

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
    let mut canvas = Canvas::new_color(900, 550, color::WHITE);

    let init_projectile = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.8, 0.0).normalize() * 10.4,
    };
    let environment = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut projectile: Projectile = init_projectile;

    while projectile.position.y >= 0.0 {
        projectile = tick(&environment, &projectile);
        canvas.write(
            projectile.position.x as usize,
            canvas.height - projectile.position.y as usize,
            color::RED,
        );
    }
    let ppm = ppm::new(canvas);
    println!("{}", ppm);
}

fn tick(environment: &Environment, projectile: &Projectile) -> Projectile {
    Projectile {
        position: projectile.position.displacment(&projectile.velocity),
        velocity: projectile.velocity + environment.gravity + environment.wind,
    }
}
