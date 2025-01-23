use image::{Rgb, RgbImage};

pub mod math;
use crate::math::*;

struct ImageConfig {
    dimensions: (u32, u32),
    fov: (f32, f32),
}

enum Mat{
    Normal(),
    Color(Rgb<u8>),
}

struct Shapes<'a> {
    spheres: &'a[(Sphere, Mat)],
    //TODO Add Tri
}

fn main() {
    //TODO: Later make the config and shapes read in from a config file.
    //      Potentially use ChShersh's CCL?
    let config = ImageConfig {
        dimensions: (640, 480),
        fov: (90.0, 67.5),
    };

    let shapes = Shapes {
        spheres: &[
            (Sphere {
                pos: vec3(0.0, 0.0, -1.0),
                radius: 0.5,
            }, Mat::Normal()),
            (Sphere {
                pos: vec3(-0.5, -0.5, -1.0),
                radius: 0.1,
            }, Mat::Normal()),
            (Sphere {
                pos: vec3(0.5, 0.5, -1.0),
                radius: 0.1,
            }, Mat::Normal()),
        ],
    };
    _ = render(config, shapes).save("./img.png");
}

fn render(config: ImageConfig, shapes: Shapes) -> RgbImage {
    fn progress(prog: u32, total: u32) {
        let progress: f64 = 100.0 * prog as f64 / (total - 1) as f64;
        eprint!("\rProgress: {0:.2}%", progress);
    }

    let (length, height) = config.dimensions;
    let mut img = RgbImage::new(length, height);
    for y in 0..height {
        for x in 0..length {
            let color = raytrace(get_ray(&config, x, y), &shapes);
            img.put_pixel(x, y, color);
        }
        progress(y, height);
    }
    img
}

//Gets a specific pixel's ray, according to resolution and FOV
fn get_ray(config: &ImageConfig, x: u32, y: u32) -> Ray {
    let (x_fov, y_fov) = config.fov;
    let (width, height) = config.dimensions;
    let x_axis = vec3(1.0, 0.0, 0.0);
    let y_axis = vec3(0.0, 1.0, 0.0);

    let x_rot_step = x_fov / width as f32;
    let y_rot_step = y_fov / height as f32;
    let x_rot = radians( x as f32 * x_rot_step 
        - x_rot_step * (width as f32 / 2.0) 
        + x_rot_step / 2.0);
    let y_rot = radians( y as f32 * y_rot_step 
        - y_rot_step * (height as f32 / 2.0)
        + y_rot_step / 2.0);

    let ray_dir_x_rot = rotate_vec3(&vec3(0.0, 0.0, -1.0), -1.0 * x_rot, &y_axis);
    let ray_dir = rotate_vec3(&ray_dir_x_rot, -1.0 * y_rot, &x_axis);

    Ray {
        pos: vec3(0.0, 0.0, 0.0),
        dir: ray_dir,
    }
}

fn raytrace(r: Ray, shapes: &Shapes) -> Rgb<u8> {
    for s in shapes.spheres {
        match ray_sphere_intersect(&r, &s.0) {
            Some(intersection) => match &s.1 {
                Mat::Normal() => return vec_to_color(point_from_ray(&r, intersection) - s.0.pos),
                Mat::Color(rgb) => return *rgb,
            },
            None => continue,
        }
    }
    Rgb([0, 0, 0])
}

fn point_from_ray(r: &Ray, t: f32) -> Vec3 {
    (t * r.dir) + r.pos
}

fn vec_to_color(v: Vec3) -> Rgb<u8> {
    let v2 = 0.5 * normalize(&v) + vec3(0.5, 0.5, 0.5);
    vec3_to_rgb(v2)
}

fn vec3_to_rgb(v: Vec3) -> Rgb<u8> {
    let v2 = 255.0 * v;
    Rgb([v2.x as u8, v2.y as u8, v2.z as u8])
}

fn _hello_world_image(x: u8, y: u8) -> Rgb<u8> {
    let y_clamp = if y > x { x } else { y };
    Rgb([255 - x, y, x - y_clamp])
}
