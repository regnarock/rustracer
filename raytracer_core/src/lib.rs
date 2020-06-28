#![feature(tau_constant)]
#![feature(clamp)]
#![feature(associated_type_bounds)]

use std::thread;

pub use nalgebra::Vector3;

use crate::camera::Camera;
use crate::rand_range_f64::rand_range_f64;
use crate::rand_range_f64::shuffle;
use crate::shapes::shape::Shape;
use crate::shapes::sphere::Sphere;

mod camera;
mod materials;
mod rand_range_f64;
pub mod shapes;

/// Imported from renderers:

#[derive(Debug, Clone, Copy)]
pub struct PixelColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl From<Vector3<f64>> for PixelColor {
    fn from(vector3: Vector3<f64>) -> Self {
        Self {
            r: vector3.x as u8,
            g: vector3.y as u8,
            b: vector3.z as u8,
        }
    }
}
pub struct PixelPosition {
    pub x: usize,
    pub y: usize,
}
pub type PixelAccessor = Fn(PixelPosition, PixelColor) + Send;
///

pub type Scene<'a> = Vec<&'a dyn Shape>;

pub struct Raytracer {

}
impl Raytracer {
    pub fn generate<T, R>(&self, width: f64, height: f64, scene: Scene, SAMPLES_PER_PIXEL: i64, set_pixel: T, mut random: R)
            where T: Fn(PixelPosition, PixelColor) + Send,
            R: rand::Rng + 'static + Send {
        rand_range_f64::init_RNG(move |start, end| {
            return random.gen_range(start, end)
        });
        let camera = Camera::new();
        let random_positions = all_pixels_at_random(height as i64, width as i64);
        let scale = 1.0 / SAMPLES_PER_PIXEL as f64;
        for pos in random_positions {
            let mut samples_color = Vector3::new(0.0, 0.0, 0.0);
            for _s in 0..SAMPLES_PER_PIXEL {
                let offset_x = (pos.x as f64 + rand_range_f64(0.0, 1.0)) / (width - 1.0);
                let offset_y = (pos.y as f64 + rand_range_f64(0.0, 1.0)) / (height - 1.0);
                let r = camera.emit_ray_at(offset_x, offset_y);
                samples_color += r.project_ray(&scene);
            }
    
            let scale = 1.0 / SAMPLES_PER_PIXEL as f64;
            let corrected_pixel_color = (samples_color * scale)
                .map(|c| c.clamp(0.0, 1.0))
                .map(f64::sqrt)
                .map(|c| c * 255.0);
            set_pixel(pos, PixelColor::from(corrected_pixel_color));
        }
    }

}

fn all_pixels_at_random(height: i64, width: i64) -> Vec<PixelPosition> {
    let mut random_y: Vec<i64> = (0..height).rev().collect();
    let mut random_x: Vec<i64> = (0..width).rev().collect();
    shuffle(random_y.as_mut_slice());
    let mut random_positions: Vec<PixelPosition> = random_y
        .iter()
        .flat_map(|y| -> Vec<PixelPosition> {
            shuffle(random_x.as_mut_slice());
            random_x
                .iter()
                .map(|x| -> PixelPosition {
                    PixelPosition {
                        y: *y as usize,
                        x: *x as usize,
                    }
                })
                .collect()
        })
        .collect();
    shuffle(random_positions.as_mut_slice());
    random_positions
}