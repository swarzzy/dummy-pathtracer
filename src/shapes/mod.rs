use crate::math::*;
use crate::materials::*;

pub mod sphere;

pub use sphere::*;

#[derive(Copy, Clone)]
pub struct HitInfo<'a> {
    pub ray: Ray,
    pub t: f32,
    pub p: v3,
    pub normal: v3,
    pub material: &'a Material
}

pub trait Hit {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitInfo>;
}
