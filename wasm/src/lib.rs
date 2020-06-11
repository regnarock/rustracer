// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate.
use wasm_bindgen::prelude::*;

use raytracer_core::Vector3;
use raytracer_core::shapes::sphere::Sphere;
use raytracer_core::{Scene, Raytracer, PixelColor, PixelPosition};
use rand::prelude::*;
// use rand_core::{RngCore, OsRng};

// Define the size of our camera
const SIZE: usize = 200;

const SAMPLES_PER_PIXEL: i64 = 64;

/*
 * 1. What is going on here?
 * Create a static mutable byte buffer.
 * We will use for putting the output of our graphics,
 * to pass the output to js.
 * NOTE: global `static mut` means we will have "unsafe" code
 * but for passing memory between js and wasm should be fine.
 *
 * 2. Why is the size SIZE * SIZE * 4?
 * We want to have 200 pixels by 200 pixels. And 4 colors per pixel (r,g,b,a)
 * Which, the Canvas API Supports.
 */
const OUTPUT_BUFFER_SIZE: usize = SIZE * SIZE * 4;
static mut OUTPUT_BUFFER: [u8; OUTPUT_BUFFER_SIZE] = [0; OUTPUT_BUFFER_SIZE];

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

// Function to return a pointer to our buffer
// in wasm memory
#[wasm_bindgen]
pub fn get_output_buffer_pointer() -> *const u8 {
  let pointer: *const u8;
  unsafe {
    pointer = OUTPUT_BUFFER.as_ptr();
  }

  return pointer;
}

fn set_pixel(position: PixelPosition, c: PixelColor) {
  // Let's calculate our index, using our 2d -> 1d mapping.
  // And then multiple by 4, for each pixel property (r,g,b,a).
  let square_number: usize = (SIZE * SIZE) - (position.y * SIZE + position.x) - 1;
  let square_rgba_index: usize = square_number * 4;

  unsafe {
    OUTPUT_BUFFER[square_rgba_index + 0] = c.r; // Red
    OUTPUT_BUFFER[square_rgba_index + 1] = c.g; // Green
    OUTPUT_BUFFER[square_rgba_index + 2] = c.b; // Blue
    OUTPUT_BUFFER[square_rgba_index + 3] = 255; // Alpha (Always Opaque)
  }
}

fn get_random_f64(inclusive: f64, exclusive: f64) -> f64 {
  rand::thread_rng().gen_range(inclusive, exclusive)
}

// Function to generate our checkerboard, pixel by pixel
#[wasm_bindgen]
pub fn render() {
      let sphere = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5);
      let sphere2 = Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0);
      let sphere3 = Sphere::new(Vector3::new(0.5, -0.4, -0.85), 0.1);
      let scene: Scene = vec![&sphere, &sphere2, &sphere3];
      let raytracer = Raytracer{};
      raytracer.generate(SIZE as f64, SIZE as f64, scene, SAMPLES_PER_PIXEL, set_pixel, get_random_f64);
}