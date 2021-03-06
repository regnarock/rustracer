use crate::shapes::ray::Ray;
use nalgebra::Rotation3;
use nalgebra::Vector3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;

pub struct Camera {
    pub origin: Vector3<f64>,
    lookat: Vector3<f64>,
    lower_left_corner: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
}

impl Camera {
    pub fn new(x: f64, y: f64, z: f64) -> Camera {
        Self::new_lookat(x, y, z, Vector3::new(0.0, 0.0, -1.0))
    }
    pub fn new_lookat(x: f64, y: f64, z: f64, lookat: Vector3<f64>) -> Camera {
        let origin = Vector3::new(x, y, z);
        let vup = Vector3::new(0.0, 1.0, 0.0);
        let vertical_field_of_view = 20_f64;

        let viewport_height: f64 = 2.0 * vertical_field_of_view.to_radians();
        let viewport_width: f64 = ASPECT_RATIO * viewport_height;

        let w = (origin - lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;

        Camera {
            origin,
            lookat,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - w,
        }
    }

    pub fn emit_ray_at(&self, offset_x: f64, offset_y: f64) -> Ray {
        Ray::new(
            self.origin.clone_owned(),
            self.lower_left_corner - self.origin
                + self.horizontal * offset_x
                + self.vertical * offset_y,
        )
    }

    pub fn rotate(&self, rotation: Vector3<f64>) -> Self {
        let mut look_at_offset = self.lookat - self.origin;
        //look_at_offset.y = 0.0;
        let rotation = Rotation3::from_euler_angles(rotation.x, rotation.y, rotation.z);
        look_at_offset = rotation * look_at_offset;
        Self::new_lookat(
            self.origin.x,
            self.origin.y,
            self.origin.z,
            self.origin + look_at_offset,
        )
    }
    pub fn move_camera(&self, dir: Vector3<f64>) -> Self {
        let mut look_at_offset = self.lookat - self.origin;
        look_at_offset.y = 0.0;
        let vup = Vector3::new(0.0, 1.0, 0.0);
        let rotation = Rotation3::face_towards(&look_at_offset, &vup);
        let real_dir = rotation.transform_vector(&dir);

        //let real_dir = rotation;
        Self::new_lookat(
            self.origin.x + real_dir.x,
            self.origin.y + real_dir.y,
            self.origin.z + real_dir.z,
            self.lookat + real_dir,
        )
    }
}
