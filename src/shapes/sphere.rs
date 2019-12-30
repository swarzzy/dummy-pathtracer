use crate::math::*;
use crate::shapes::*;

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: v3,
    pub r: f32,
    pub material: Material,
}

impl Sphere {
    pub fn make(c: v3, r: f32, mat: Material) -> Self {
        Self {
            center: c,
            r: r,
            material: mat
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitInfo> {

        let ac = ray.origin - self.center;
        let a = dot(ray.dir, ray.dir);
        let b = 2.0 * dot(ac, ray.dir);
        let c = dot(ac, ac) - self.r * self.r;
        let discr = b * b - 4.0 * a * c;
        if discr < 0.0 {
            None
        } else {
            let t1 = (-b - discr.sqrt()) / (2.0 * a);
            let t2 = (-b + discr.sqrt()) / (2.0 * a);

            let mut result = None;

            if (t_min..t_max).contains(&t1) {
                let mut t_result = t1;
                if (t_min..t_max).contains(&t2) && t2 < t1 {
                    t_result = t2;
                }

                let hit_point = ray.travel(t_result);

                result = Some(HitInfo {
                    ray: ray,
                    t: t_result,
                    p: hit_point,
                    normal: (hit_point - self.center).normalize(),
                    material: &self.material
                })
            }
            result
        }
    }
}
