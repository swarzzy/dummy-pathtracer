pub mod vec;
pub mod ray;

pub use self::vec::v3;
pub use self::ray::Ray;

extern crate rand;
use rand::Rng;

#[allow(non_snake_case)]
pub fn V3(x: f32, y: f32, z: f32) -> v3 {
    v3::make(x, y, z)
}

pub fn lerp(a: v3, b: v3, t: f32) -> v3 {
    (1.0 - t) * a + t * b
}

pub fn dot(a: v3, b: v3) -> f32 {
    a.dot(b)
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min { min }
    else if x > max { max }
    else { x }
}

pub fn refract(v: v3, n: v3, ior: f32) -> (bool, v3) {
    // NOTE: Copy-pasted from book
    let vn = v.normalize();
    let vn_dot_n = vn.dot(n);
    let discr = 1.0 - ior * ior * (1.0 - vn_dot_n * vn_dot_n);
    if discr > 0.0 {
        let refracted = ior * (vn - n * vn_dot_n) - n * discr.sqrt();
        (true, refracted)
    } else {
        (false, Default::default())
    }
}

pub fn fresnel_schlick(cos: f32, ior: f32) -> f32 {
    let r0 = (1.0 - ior) / (1.0 + ior);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
}

pub fn random_unilateral() -> f32 {
    rand::thread_rng().gen_range(0.0, 1.0)
}

pub fn random_unilateral_v3() -> v3 {
    v3 {
        x: random_unilateral(),
        y: random_unilateral(),
        z: random_unilateral()
    }
}

pub fn random_on_unit_sphere() -> v3 {
    let result;
    loop {
        let p = 2.0 * V3(random_unilateral(), random_unilateral(), random_unilateral()) - V3(1.0, 1.0, 1.0);
        if p.len_sq() < 1.0 {
            result = p;
            break;
        }
    }
    result
}

pub fn linear_to_srgb(linear: v3) -> v3 {
    v3 {
        x: linear.x.sqrt(),
        y: linear.y.sqrt(),
        z: linear.z.sqrt(),
    }
}
