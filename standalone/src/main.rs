#![feature(tau_constant)]
#![feature(clamp)]

use std::thread;

use raytracer_core::Vector3;

use crate::renderers::pixels::RendererPixels;
use crate::renderers::renderer::{Dimensions, Renderer};
use raytracer_core::shapes::sphere::Sphere;
use raytracer_core::{Raytracer, Scene};

use rand::rngs::SmallRng;
use rand::SeedableRng;

mod renderers;

const SAMPLES_PER_PIXEL: i64 = 50;

fn main_loop() {
    let width = 1920.0 / 2.0;
    let height = 1080.0 / 2.0;

    let mut renderer = RendererPixels::new(Dimensions {
        height: height as usize,
        width: width as usize,
    });
    let set_pixel = renderer.pixel_accessor();
    eprint!("Scanlines remaining:\n");
    thread::spawn(move || {
        let sphere = Sphere::new_with_metal(Vector3::new(0.0, 0.0, -1.0), 0.5);
        let sphere2 = Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0);
        let sphere3 = Sphere::new(Vector3::new(0.5, -0.4, -0.85), 0.1);
        let scene: Scene = vec![&sphere, &sphere2, &sphere3];
        let rng = &mut SmallRng::from_entropy();

        let raytracer = Raytracer::new(width, height, rng);

        for _depth in 0..=SAMPLES_PER_PIXEL {
            raytracer.generate(scene.as_slice(), 1, &set_pixel, rng);
        }
        eprintln!("OK");
    });
    renderer.start_rendering();
}

fn main() -> std::io::Result<()> {
    main_loop();
    Ok(())
}
