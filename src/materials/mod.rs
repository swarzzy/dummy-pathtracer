use crate::math::*;
use crate::shapes::*;

#[derive(Copy, Clone, Default)]
pub struct ScatterInfo {
    pub absorbed: bool,
    pub attenuation: v3,
    pub scattered: Ray
}

pub trait Scatter {
    fn scatter(&self, hit_info: &HitInfo) -> ScatterInfo;
}

#[derive(Copy, Clone, Default)]
pub struct LambertianMaterial {
    pub albedo: v3
}

#[derive(Copy, Clone, Default)]
pub struct MetalMaterial {
    pub albedo: v3,
    pub roughness: f32
}

#[derive(Copy, Clone, Default)]
pub struct DielectricMaterial {
    pub albedo: v3,
    pub ior: f32
}

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(LambertianMaterial),
    Metal(MetalMaterial),
    Dielectric(DielectricMaterial)
}

impl Scatter for Material {
    fn scatter(&self, hit_info: &HitInfo) -> ScatterInfo {
        match self {
            Material::Lambertian(m) => m.scatter(hit_info),
            Material::Metal(m) => m.scatter(hit_info),
            Material::Dielectric(m) => m.scatter(hit_info),
        }
    }
}

impl LambertianMaterial {
    pub fn make(albedo: v3) -> Material {
        Material::Lambertian(Self {
            albedo
        })
    }
}

impl MetalMaterial {
    pub fn make(albedo: v3, roughness: Option<f32>) -> Material {
        Material::Metal(Self {
            albedo: albedo,
            roughness: roughness.unwrap_or(1.0)
        })
    }
}

impl DielectricMaterial {
    pub fn make(albedo: v3, ior: Option<f32>) -> Material {
        Material::Dielectric(Self {
            albedo: albedo,
            ior: ior.unwrap_or(1.0)
        })
    }
}


impl Scatter for MetalMaterial {
    fn scatter(&self, hit: &HitInfo) -> ScatterInfo {
        let refl = hit.ray.dir.normalize().reflect(hit.normal);
        let scattered = Ray::make(hit.p, refl + self.roughness * random_on_unit_sphere());
        ScatterInfo {
            absorbed: !(scattered.dir.dot(hit.normal) > 0.0),
            attenuation: self.albedo,
            scattered: scattered
        }
    }
}

impl Scatter for LambertianMaterial {
    fn scatter(&self, hit: &HitInfo) -> ScatterInfo {
        let target = hit.p + hit.normal + random_on_unit_sphere();
        let scattered = Ray::make(hit.p, target - hit.p);
        ScatterInfo {
            absorbed: false,
            attenuation: self.albedo,
            scattered: scattered
        }
    }
}
impl Scatter for DielectricMaterial {
    fn scatter(&self, hit: &HitInfo) -> ScatterInfo {
        let reflected = hit.ray.dir.reflect(hit.normal);
        let out_normal;
        let ior;
        let scattered;
        let absorbed = false;
        let attenuation = self.albedo;
        let cos;

        if hit.ray.dir.dot(hit.normal) > 0.0 {
            out_normal = -hit.normal;
            ior = self.ior;
            cos = ior * hit.ray.dir.dot(hit.normal) / hit.ray.dir.len();
        } else {
            out_normal = hit.normal;
            ior = 1.0f32 / self.ior;
            cos = -(hit.ray.dir.dot(hit.normal)) / hit.ray.dir.len();
        }

        let (refr, refr_dir) = refract(hit.ray.dir, out_normal, ior);
        let refl_prob;
        if refr {
            refl_prob = fresnel_schlick(cos, ior);
        } else {
            refl_prob = 1.0;
        }

        if random_unilateral() < refl_prob {
            scattered = Ray::make(hit.p, reflected);
        } else {
            scattered = Ray::make(hit.p, refr_dir);
        }

        ScatterInfo {
            absorbed, attenuation, scattered
        }
    }
}
