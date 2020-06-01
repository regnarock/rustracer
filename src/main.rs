#![feature(tau_constant)]
#![feature(clamp)]

use nalgebra::Vector3;

use crate::camera::Camera;
use crate::rand_range_f64::rand_range_f64;
use crate::renderer::Color;
use crate::shapes::shape::Shape;
use crate::shapes::sphere::Sphere;

mod camera;
mod collision;
mod materials;
mod rand_range_f64;
mod ray;
mod renderer;
mod shapes;

const SAMPLES_PER_PIXEL: i64 = 4;

fn main_loop() {
    let camera = Camera::new();
    let width = 640.0;
    let height = 360.0;

    let sphere = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5);
    let sphere2 = Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0);
    let sphere3 = Sphere::new(Vector3::new(0.5, -0.4, -0.85), 0.1);
    let scene: Vec<&dyn Shape> = vec![&sphere, &sphere2, &sphere3];
    let mut renderer = renderer::RendererPPM::new(height as usize, width as usize);

    eprint!("Scanlines remaining:\n");
    for y in (0..(height as i64)).rev() {
        eprint!("\r{} <= {}", height, height as i64 - y);
        for x in 0..(width as i64) {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..SAMPLES_PER_PIXEL {
                let offset_x = (x as f64 + rand_range_f64(0.0, 1.0)) / (width - 1.0);
                let offset_y = (y as f64 + rand_range_f64(0.0, 1.0)) / (height - 1.0);
                let r = camera.emit_ray_at(offset_x, offset_y);
                pixel_color += r.project_ray(&scene);
            }

            let scale = 1.0 / SAMPLES_PER_PIXEL as f64;
            let corrected_pixel_color = (pixel_color * scale)
                .map(|c| c.clamp(0.0, 1.0))
                .map(f64::sqrt);
            renderer.set_pixel(x as usize, y as usize, corrected_pixel_color);
        }
    }

    eprint!("\nDone! :-)\n");
    renderer.render();
}

fn main() -> std::io::Result<()> {
    main_loop();
    Ok(())
}
