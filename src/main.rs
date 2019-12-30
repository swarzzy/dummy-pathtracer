#![allow(dead_code)]

use std::fs::File;
use std::io::Write;

mod math;
mod shapes;
mod materials;
mod camera;

use math::*;
use shapes::*;
use materials::*;
use camera::*;

fn color<T>(geometry: &Vec<T>, r : Ray, depth: i32) -> v3 where T: Hit {

    let mut closest_hit: Option<HitInfo> = None;
    for shape in geometry.iter() {
        shape.hit(r, 0.001, std::f32::MAX).map(|result| {
            match closest_hit {
                Some(hit) => {
                    if result.t < hit.t {
                        closest_hit = Some(result);
                    }
                },
                None => closest_hit = Some(result)
            };
        });
    }

    match closest_hit {
        Some(hit) => {
            let scatter_info = hit.material.scatter(&hit);
            if !scatter_info.absorbed && depth < 50 {
                scatter_info.attenuation.hadamard(color(geometry, scatter_info.scattered, depth + 1))
            } else {
                V3(0.0, 0.0, 0.0)
            }
        },
        None => {
            let dir = r.dir.normalize();
            let t: f32 = 0.5 * dir.y + 1.0;
            lerp(V3(1.0, 1.0, 1.0), V3(0.5, 0.7, 1.0), t)
        }

    }
}

fn make_scene<'a>() -> Vec<Sphere> {
    let mut scene = Vec::new();
    scene.push(Sphere::make(V3(0.0, -1000.0, 0.0), 1000.0, LambertianMaterial::make(V3(0.5, 0.5, 0.5))));
    for a in -11..11 {
        for b in -11..11 {
            let rand_val = random_unilateral();
            let center = V3(a as f32 + 0.9 * random_unilateral(), 0.2, b as f32 + 0.9 * random_unilateral());
            if (center - V3(4.0, 0.2, 0.0)).len() > 0.9 {
                if rand_val < 0.8 {
                    scene.push(Sphere::make(center, 0.2, LambertianMaterial::make(random_unilateral_v3())));
                }
                else if rand_val < 0.95 {
                    scene.push(Sphere::make(center, 0.2, MetalMaterial::make(random_unilateral_v3(), Some(0.5 * random_unilateral()))));
                }
                else {
                    scene.push(Sphere::make(center, 0.2, DielectricMaterial::make(V3(1.0, 1.0, 1.0), Some(1.5))));
                }
            }
        }
    }

    scene.push(Sphere::make(V3(0.0, 1.0, 0.0), 1.0, DielectricMaterial::make(V3(1.0, 1.0, 1.0), Some(1.5))));
    scene.push(Sphere::make(V3(-4.0, 1.0, 0.0), 1.0, LambertianMaterial::make(V3(0.4, 0.2, 0.1))));
    scene.push(Sphere::make(V3(4.0, 1.0, 0.0), 1.0, MetalMaterial::make(V3(0.7, 0.6, 0.5), Some(0.0))));

    scene
}

fn main() {
    let width = 1920;
    let height = 1080;
    let spp = 30;

    let mut file = File::create("out.ppm").unwrap();
    write!(&mut file, "P3\n{} {}\n255\n", width, height).unwrap();

    let spheres = make_scene();
    let camera = Camera::default();

    for y in (0..height).rev() {
        let percent = (height - y) as f32 / height as f32;
        println!("Path tracing... {}%", (percent * 100.0) as i32);
        for x in 0..width {
            let mut pixel_color = v3::default();
            for _ in  0..spp {
                let u = (x as f32 + random_unilateral()) / width as f32;
                let v = (y as f32 + random_unilateral()) / height as f32;

                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + color(&spheres, ray, 0);
            }
            let pixel_color = pixel_color / spp as f32;
            let pixel_color = linear_to_srgb(pixel_color);

            let r = (255.99f32 * pixel_color.x) as i32;
            let g = (255.99f32 * pixel_color.y) as i32;
            let b = (255.99f32 * pixel_color.z) as i32;

            write!(&mut file, "{} {} {}\n", r, g, b).unwrap();
        }
    }
}
