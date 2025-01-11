use image::{Rgb, RgbImage};
use lina::{vec3, Vec3f};

struct ImageConfig {
    dimensions: (u32, u32),
    fov: (u32, u32),
}

struct Sphere {
    loc: Vec3f,
    radius: f32,
}

struct Shapes {
    spheres: Vec<Sphere>,
    //TODO Add Tri
}

fn main() {
    //TODO: Later make the config and shapes read in from a config file.
    //      Potentially use ChShersh's CCL?
    let config = ImageConfig {
        dimensions: (640, 480),
        fov: (120, 90),
    };

    let shapes = Shapes {
        spheres: vec![Sphere {
            loc: vec3(0.0, 0.0, 1.0),
            radius: 0.5,
        }],
    };
    _ = render(config, shapes).save("./img.png");
}

struct Ray {
    loc: Vec3f,
    dir: Vec3f,
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

fn get_ray(config: &ImageConfig, x: u32, y: u32) -> Ray {
    //TODO Implement
    Ray {
        loc: vec3(0.0, 0.0, 0.0),
        dir: vec3(0.0, 0.0, 0.0),
    }
}

fn raytrace(r: Ray, shapes: &Shapes) -> Rgb<u8> {
    //TODO Implement
    Rgb([0, 0, 0])
}

fn _hello_world_image(x: u8, y: u8) -> Rgb<u8> {
    let y_clamp = if y > x { x } else { y };
    Rgb([255 - x, y, x - y_clamp])
}
