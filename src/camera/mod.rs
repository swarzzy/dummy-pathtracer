const PI_32: f32 = 3.14159265358979323846;

use crate::math::*;

#[derive(Copy, Clone)]
pub struct Camera {
    pub p: v3,
    pub origin: v3,
    pub x_axis: v3,
    pub y_axis: v3,
    pub lens_r: f32,
    pub x_base_axis: v3,
    pub y_base_axis: v3,
    pub z_base_axis: v3,
}

impl Default for Camera {
    fn default() -> Camera {
        let from = V3(13.0, 2.0, 3.0);
        let target = V3(0.0, 0.0, 0.0);
        //let focus_dist = (from - ta rget).len();
        Camera::make(from, target, V3(0.0, 1.0, 0.0), 20.0, 16.0 / 9.0, 0.1, 10.0)
    }
}

impl Camera {
    pub fn make(from: v3, target: v3, up: v3, fov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Camera {
        let lens_r = aperture / 2.0;

        let theta = fov * (PI_32 / 180.0);
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let p = from;
        let z_base_axis = (from - target).normalize();
        let x_base_axis = up.cross(z_base_axis).normalize();
        let y_base_axis = z_base_axis.cross(x_base_axis).normalize();

        let origin = p
            - half_width * x_base_axis * focus_dist
            - half_height * y_base_axis * focus_dist
            - z_base_axis * focus_dist;

        let x_axis = 2.0 * half_width * x_base_axis * focus_dist;
        let y_axis = 2.0 * half_height * y_base_axis * focus_dist;

        Camera {
            p, origin, x_axis, y_axis, lens_r, x_base_axis, y_base_axis, z_base_axis
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rand_p = self.lens_r * random_on_unit_sphere();
        let rand_p = V3(rand_p.x, rand_p.y, 0.0);
        let offset = self.x_base_axis * rand_p.x + self.y_base_axis * rand_p.y;
        Ray {
            origin: self.p + offset,
            dir: self.origin + u * self.x_axis + v * self.y_axis - self.p - offset
        }
    }
}
