#![allow(dead_code)]

use std::fs::File;
use std::io::Write;
use std::io;

use std::mem::size_of;

mod math;
mod shapes;
mod materials;
mod camera;

use math::*;
use shapes::*;
use materials::*;
use camera::*;

#[derive(Copy, Clone, Default)]
#[repr(C, packed)]
struct BitmapHeader {
    file_type: u16,
    file_size: u32,
    reserved1: u16,
    reserved2: u16,
    bitmap_offset: u32,
    size: u32,
    width: i32,
    height: i32,
    planes: u16,
    bits_per_pixel: u16,
    compression: u32,
    size_of_bitmap: u32,
    h_resolution: i32,
    v_resolution: i32,
    colors_used: u32,
    colors_important: u32
}

impl  BitmapHeader {
    fn make() -> Self {
        Self {
            file_type: 0x4d42,
            size: (size_of::<Self>() - 14) as u32,
            bitmap_offset: size_of::<Self>() as u32,
            planes: 1,
            bits_per_pixel: 32,
            compression: 0,
            .. Default::default()
        }
    }
}

#[derive(Default)]
struct Image {
    width: u32,
    height: u32,
    pixels: Vec<u32>
}

impl Image {
    fn make(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![0; (width * height) as usize]
        }
    }

    fn pixel(&mut self, x: u32, y: u32) -> Option<&mut u32> {
        self.pixels.get_mut((y * self.width + x) as usize)
    }

    fn write_as_bmp(&self, filename: &str) -> io::Result<()> {
        let pixels_size = self.width * self.height * size_of::<u32>() as u32;
        assert!(pixels_size == (self.pixels.len() * size_of::<u32>()) as u32);

        let mut header = BitmapHeader::make();
        header.file_size = size_of::<BitmapHeader>() as u32 + pixels_size;
        header.width = self.width as i32;
        header.height = self.height as i32;
        header.size_of_bitmap = pixels_size;

        let mut file = File::create(filename)?;

        let header_ptr: *const u8 = &header as *const _ as *const u8;
        let header_size = size_of::<BitmapHeader>();
        let header_slice = unsafe { std::slice::from_raw_parts(header_ptr, header_size) };
        file.write_all(header_slice)?;

        let bitmap_ptr = self.pixels.as_ptr() as *const u8;
        let bitmap_slice = unsafe { std::slice::from_raw_parts(bitmap_ptr, pixels_size as usize) };
        file.write_all(bitmap_slice)?;
        Ok(())
    }
}

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
    let width = 200;
    let height = 100;
    let spp = 15;

    let mut image = Image::make(width, height);

//    let mut file = File::create("out.ppm").unwrap();
//    write!(&mut file, "P3\n{} {}\n255\n", width, height).unwrap();

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

            let r = (255.99f32 * pixel_color.x) as u32;
            let g = (255.99f32 * pixel_color.y) as u32;
            let b = (255.99f32 * pixel_color.z) as u32;

            let pixel = image.pixel(x, y).unwrap();
            *pixel = (r << 16) | (g << 8) | b;

            image.write_as_bmp("out.bmp").unwrap();
//            write!(&mut file, "{} {} {}\n", r, g, b).unwrap();
        }
    }
}
