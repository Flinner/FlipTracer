
# FlintTracer

This is my ray tracer that I wrote while following [The Ray Tracer Challenge](http://raytracerchallenge.com/)!

This project didn't use any external library/crate, everything was done from scratch!

> Brace yourself for a fun challenge: build a photorealistic 3D renderer from scratch!  
> 
> It’s easier than you think. In just a couple of weeks, build a ray
> tracer that renders beautiful scenes with shadows, reflections,
> brilliant refraction effects, and subjects composed of various
> graphics primitives: spheres, cubes, cylinders, triangles, and more.

To run it, do:

```
cargo run --bin {scene-name} | feh - 

```

where the scene is one of

```
    circle
    clock
    cover
    cube
    projectile
    raytracer
    scene
    transperant
```

# Supported Features

## Objects
- Cube
- Cylinder
- Cone
- Planes
- Sphere

## Materials

Any material can be formed with the following combination

```rust

     color: Color,
    /// Value between 0 and 1, default: 0.1
     ambient: f64
    /// Value between 0 and 1, default: 0.9
     diffuse: f64
    /// Value between 0 and 1, default: 0.9
     specular: f64
    /// Value between 10 and 200 work best,default: 200.0
    /// no limits apart from `f64`
     shininess: f64
    /// Value Between 0 and 1.
    /// 0 is non-reflective, 1 is a perfect mirror
     reflective: f64
    /// Transparency, 0 is opaque, 1 is (perfectly) transparent
     transparency: f64
    /// Refreactive Index
     refractive_index: f64
    /// Pattern
     pattern: Pattern,
```

Where `Pattern` can be any of:

```rs
    /// Zebra like stripes
    Stripped
    /// Gradient with two colors
    Gradient
    /// Circlur Stripes
    Ring
    /// Checkers, like Chess, but in 3D space too
    Checker
    /// The Color is the Point
    NoPattern
```

## Full fledged math library
Yep, no external crates at all!


# Example Scenes:



