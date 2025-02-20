use image::{Rgb, RgbImage};

pub mod math;
use crate::math::*;

struct ImageConfig {
    dimensions: (u32, u32),
    view: View,
}

enum Material {
    Normal(),
    Color(Rgb<u8>),
}

enum View {
    Perspective(f32, f32),
    Parallel(f32, f32),
}

enum Geometry {
    Sphere(math::Sphere),
    Triangle(math::Triangle),
}

struct Shape {
    mat: Material,
    geo: Geometry,
}

impl Shape {
    fn intersect(&self, r: &Ray) -> Option<f32> {
        match &self.geo {
            Geometry::Sphere(sphere) => ray_sphere_intersect(r, sphere),
            Geometry::Triangle(tri) => ray_triangle_intersect(r, tri),
        }
    }
}

fn main() {
    //TODO: Later make the config and shapes read in from a config file.
    //      Potentially use ChShersh's CCL?
    let config = ImageConfig {
        dimensions: (640, 480),
        //view: View::Parallel(2.0, 1.5),
        view: View::Perspective(90.0, 67.5),
    };

    let shapes = vec![
        Shape {
            geo: Geometry::Sphere(Sphere {
                pos: math::vec3(0.0, 0.0, -2.0),
                radius: 0.5,
            }),
            mat: Material::Normal(),
        },
        Shape {
            geo: Geometry::Sphere(Sphere {
                pos: math::vec3(-0.5, -0.5, -2.0),
                radius: 0.1,
            }),
            mat: Material::Normal(),
        },
        Shape {
            geo: Geometry::Sphere(Sphere {
                pos: math::vec3(0.5, 0.5, -2.0),
                radius: 0.1,
            }),
            mat: Material::Normal(),
        },
        Shape {
            geo: Geometry::Triangle(Triangle(
                         vec3(-0.5, 0.6, -2.0),
                         vec3(-0.3, 0.0, -1.5),
                         vec3(-0.1, 0.6, -2.0),
                 )),
            mat: Material::Color(Rgb([255, 100, 100])),
        },
        Shape {
            geo: Geometry::Triangle(Triangle(
                         vec3(0.5, -0.6, -2.0),
                         vec3(0.3, 0.0, -1.5),
                         vec3(0.1, -0.6, -2.0),
                 )),
            mat: Material::Color(Rgb([100, 150, 255])),
        },
    ];
    _ = render(config, shapes).save("./img.png");
}

fn render(config: ImageConfig, shapes: Vec<Shape>) -> RgbImage {
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
    let get_ray_persp = |x_fov, y_fov| -> Ray {

        let (width, height) = config.dimensions;
        let x_axis = vec3(1.0, 0.0, 0.0);
        let y_axis = vec3(0.0, 1.0, 0.0);

        let top_left =
            rotate_vec3( &vec3(0.0, 0.0, -1.0), radians(y_fov / 2.0), &x_axis) +
            rotate_vec3( &vec3(0.0, 0.0, -1.0), radians(x_fov / 2.0), &y_axis);

        let x_step = (-2.0 * top_left.x) / width as f32;
        let y_step = (-2.0 * top_left.y) / height as f32;
        let ray_dir = top_left + 
            vec3((x as f32 + 0.5)*x_step, 0.0, 0.0) +
            vec3(0.0, (y as f32 + 0.5)*y_step, 0.0);

        Ray {
            pos: vec3(0.0, 0.0, 0.0),
            dir: normalize(&ray_dir),
        }
    };

    let get_ray_parallel = |view_x, view_y| -> Ray {
        let (width, height) = config.dimensions;
        let x_step = view_x / width as f32;
        let y_step = view_y / height as f32;
        let final_x = (-0.5 * view_x) + (x as f32 + 0.5) * x_step;
        let final_y = (0.5 * view_y) - (y as f32 + 0.5) * y_step;
        Ray {
            pos: vec3(final_x, final_y, 0.0),
            dir: vec3(0.0, 0.0, -1.0),
        }
    };

    match config.view {
        View::Parallel(view_x, view_y) => get_ray_parallel(view_x, view_y),
        View::Perspective(xfov, yfov) => get_ray_persp(xfov, yfov),
    }
}


fn raytrace(r: Ray, shapes: &[Shape]) -> Rgb<u8> {
    let mut intersections = Vec::<(f32, &Shape)>::new();
    for s in shapes.iter() {
        if let Some(intersection) = s.intersect(&r) {
            intersections.push((intersection, &s));
        }
    }
    //Find the first shape hit
    let Some((first_intersection, first_shape)) = intersections.into_iter() 
        .min_by(|(a, _), (b, _)| a.total_cmp(b))
    else {
        //No shape found, early return a black pixel
        return Rgb([0, 0, 0])
    };

    match first_shape.mat {
        Material::Normal() => match &first_shape.geo {
            Geometry::Sphere(sphere) => vec_to_color(point_from_ray(&r, first_intersection) - sphere.pos),
            Geometry::Triangle(_) => todo!(),
        },
        Material::Color(rgb) => rgb,
    }
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
