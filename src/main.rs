#[macro_use]
extern crate lazy_static;
extern crate image;

mod math;
use math::matrix::*;
use math::point::*;
use math::ray::*;
use math::sphere::*;
use math::vector::*;
use std::f32;

const FOV: f32 = 90_f32;

lazy_static! {
    static ref FOV_FACTOR: f32 = { 1_f32 / (FOV / 2_f32).tan() };
}

fn get_camera_ray(u: f32, v: f32, view_matrix: Matrix3) -> Ray {
    let origin = Point3::zero();
    let direction = Vector3::new(u, v, *FOV_FACTOR).normalised();

    view_matrix * Ray::new(origin, direction)
}

fn get_nearest(spheres: &Vec<Sphere>, ray: Ray) -> (bool, f32, Sphere) {
    let mut nearest_distance = f32::INFINITY;
    let mut nearest_sphere = Sphere::new(Point3::zero(), 0_f32);
    let mut is_intersection = false;

    for sphere in spheres.iter() {
        let (has_intersection, distance) = sphere.intersect(ray);

        if has_intersection && distance < nearest_distance {
            nearest_distance = distance;
            nearest_sphere = *sphere;
            is_intersection = true;
        }
    }

    (is_intersection, nearest_distance, nearest_sphere)
}

fn main() {
    let mut spheres: Vec<Sphere> = Vec::new();
    spheres.push(Sphere::new(Point3::new(0_f32, 0_f32, 3_f32), 1_f32));

    let width = 800;
    let height = 800;

    let mut imgbuf = image::GrayImage::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let u = (width as f32 / height as f32) * (2_f32 * x as f32 / width as f32 - 1_f32);
        let v = (2_f32 * y as f32 / height as f32) - 1_f32;

        // obtain the corresponding camera ray
        let camera_ray = get_camera_ray(u, v, Matrix3::identity());

        // perform an intersection test
        let (has_intersection, distance, sphere) = get_nearest(&spheres, camera_ray);

        if has_intersection {
            *pixel = image::Luma([255 as u8]);
        } else {
            *pixel = image::Luma([0 as u8]);
        }
    }

    imgbuf.save("fractal.png").unwrap();

    println!("Hello, world! {}", *FOV_FACTOR);
}
