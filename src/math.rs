use std::f32::consts;
use nalgebra_glm as glm;
use std::ops::Mul;
pub use nalgebra_glm::vec3;

pub type Vec3 = glm::TVec3<f32>;


#[derive(Debug)]
pub struct Ray {
    pub pos: Vec3,
    pub dir: Vec3,
}

#[derive(Debug)]
pub struct Sphere {
    pub pos: Vec3,
    pub radius: f32,
}

pub fn radians(degrees: f32) -> f32 {
    let pi: f32 = consts::PI;
    degrees * pi / 180.0
}

pub fn rotate_vec3(v: &Vec3, r: f32, axis: &Vec3) -> Vec3 {
    glm::rotate_vec3(v, r, axis)
}

pub fn square<T> (x: T) -> T where
    T: Mul<Output = T> + Copy {
    x * x
}

//Outputs the length to the closest raysphere intersect
pub fn ray_sphere_intersect(r: &Ray, s: &Sphere) -> Option<f32> {
    //From https://raytracing.github.io/books/RayTracingInOneWeekend.html#addingasphere/ray-sphereintersection
    /*let pos_diff = r.pos - s.pos;
    let a = glm::dot(&r.dir, &r.dir);
    let b = -2.0 * glm::dot(&r.dir, &pos_diff);
    let c = glm::dot(&pos_diff, &pos_diff) - square(s.radius);
    let discriminant = square(b) - 4.0 * a * c;

    if discriminant < 0.0 {
        None
    } else {
        Some((-b - discriminant.sqrt()) / (2.0*a))
    }*/


    // from raytracer-c back in college.

    let pos_diff = r.pos - s.pos;
    let denominator = glm::dot(&r.dir, &r.dir);

    let radical = square(glm::dot(&r.dir, &pos_diff)) - glm::dot(&r.dir, &r.dir) * 
        (glm::dot(&pos_diff, &pos_diff) - square(s.radius));

    if radical < 0.0 || denominator == 0.0 { return None }

    let zerovector = vec3(0.0, 0.0, 0.0);
    let top_left = glm::dot(&(zerovector - r.dir), &pos_diff);

    let ret = (top_left - radical.sqrt()) / denominator;
    //let ret_2 = (top_left + radical.sqrt()) / denominator; //Furthest intersection

    Some(ret)
}
