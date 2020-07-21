extern crate image;
mod Vec3;
mod Ray;
use image::{ImageBuffer};
use Vec3::vec3d;
use Ray::ray;

fn main() {
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: u32 = 384;
    let image_height: u32 = (image_width as f32/ aspect_ratio) as u32;
    let mut imgbuf = ImageBuffer::new(image_width, image_height);
    let origin: vec3d = vec3d{x: 0.0, y: 0.0, z: 0.0};
    // viewer attribution
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let view_point: vec3d = vec3d{x: 0.0, y: 0.0, z: 0.0};
    let view_horizon: vec3d = vec3d{x: viewport_width, y: 0.0, z: 0.0};
    let view_vert: vec3d = vec3d{x: 0.0, y: viewport_height, z: 0.0};
    let left_bot_corner: vec3d = view_point - (view_horizon / 2 as f32) - (view_vert / 2 as f32) - vec3d{x:0.0, y:0.0, z:focal_length};
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut(){
        let u = (x as f32) / ((image_width - 1) as f32);
        let v = ((image_height - y + 1) as f32) / ((image_height - 1) as f32);
        let mut r: ray = ray{point: view_point, direction: left_bot_corner + u * view_horizon + v * view_vert - origin};
        
        let mut r_direction: vec3d = r.direction.unitVec();
        let alpha = 0.5 * (r_direction.y() + 1.0);
        let r = ((1.0 - alpha) * 1.0 + alpha * 0.2) as f32;
        let g = ((1.0 - alpha) * 1.0 + alpha * 0.5) as f32;
        let b = 1.0 as f32; 
        let ir = (r * 255.999) as u8;
        let ig = (g * 255.999) as u8;
        let ib = (b * 255.999) as u8;
        *pixel = image::Rgb([ir, ig, ib]);
    }
    match imgbuf.save("_output_/output.png"){
        Ok(_) => println!("[*] Successfully saved to '_output_/output.png'"),
        Err(e) => println!("[!] Failed to save into file '_output_/output.png' with error {:?}", e)
    };
}
