use std::f32::consts;
use nalgebra_glm as glm;
use std::ops::Mul;
pub use nalgebra_glm::vec3;
pub use nalgebra_glm::vec2;

pub type Vec3 = glm::TVec3<f32>;
pub type Vec2 = glm::TVec2<f32>;


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

#[derive(Debug)]
pub struct Triangle(pub Vec3, pub Vec3, pub Vec3);

pub fn radians(degrees: f32) -> f32 {
    let pi: f32 = consts::PI;
    degrees * pi / 180.0
}

pub fn normalize(v: &Vec3) -> Vec3 {
    glm::normalize(&v)
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
    let pos_diff = s.pos - r.pos;
    let a = glm::dot(&r.dir, &r.dir);
    let h = glm::dot(&r.dir, &pos_diff);
    let c = glm::dot(&pos_diff, &pos_diff) - square(s.radius);

    let discriminant = square(h) - a*c;
    if discriminant < 0.0 || a == 0.0 { return None }

    Some((h - discriminant.sqrt()) / a)
}

//I will make/find a cleaner translation of the ray-tri intersection formula someday
//But today is not this day
pub fn ray_triangle_intersect(r: &Ray, tri: &Triangle) -> Option<f32> {
    let a = tri.0.x - tri.1.x;
    let b = tri.0.y - tri.1.y;
    let c = tri.0.z - tri.1.z;

    let d = tri.0.x - tri.2.x;
    let e = tri.0.y - tri.2.y;
    let f = tri.0.z - tri.2.z;

    let (g, h, i) = (r.dir.x, r.dir.y, r.dir.z);

    let j = tri.0.x - r.pos.x;
    let k = tri.0.y - r.pos.y;
    let l = tri.0.z - r.pos.z;

    let m = a*(e*i - h*f) + b*(g*f - d*i) + c*(d*h - e*g);
    //If m == 0, the ray direction is parallel to tri's plane. No intersection
    if m == 0.0 { 
        return None; 
    }

    //Calculate beta and gamma, ensuring that the values mean the ray hits the triangle 
    let beta = (j*(e*i - h*f) + k*(g*f - d*i) + l*(d*h - e*g)) / m;
    if beta > 1.0 || beta < 0.0 {
        return None;
    }

    let gamma = (i*(a*k - j*b) + h*(j*c - a*l) + g*(b*l - k*c)) / m;
    if gamma < 0.0 || gamma + beta > 1.0 {
        return None;
    }

    //The ray hits the triangle, calculate the distance to the intersection 
    Some( -1.0 * (f*(a*k - j*b) + e*(j*c - a*l) + d*(b*l - k*c)) / m )
}
