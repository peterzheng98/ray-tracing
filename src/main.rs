extern crate image;
mod Vec3;
mod Ray;
use image::{ImageBuffer};
use Vec3::vec3d;
use Ray::ray;

fn main() {
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: u32 = 600;
    let image_height: u32 = (image_width as f32/ aspect_ratio) as u32;
    let mut imgbuf = ImageBuffer::new(image_width, image_height);
    let origin: vec3d = vec3d{x: 0.0, y: 0.0, z: 0.0};
    // center point
    let circle_x: f32 = 300.0;
    let circle_y: f32 = 168.5;
    for (_, _, pixel) in imgbuf.enumerate_pixels_mut(){
        *pixel = image::Rgb([15 as u8, 15 as u8, 25 as u8]);
    }

    let radius: f32 = 135.0;
    let left_point = circle_x - radius;
    let right_point = circle_x + radius;
    let step: f32 = 0.001;
    let mut start_point: f32 = left_point;
    while start_point <= right_point{
        let y0_minus = ((radius * radius - ((start_point - circle_x) * (start_point - circle_x))) as f32).sqrt();
        let y0 = y0_minus + circle_y;
        let y1 = -y0_minus + circle_y;
        *imgbuf.get_pixel_mut(start_point as u32, y0 as u32) = image::Rgb([255 as u8, 255 as u8, 255 as u8]);
        *imgbuf.get_pixel_mut(start_point as u32, y1 as u32) = image::Rgb([255 as u8, 255 as u8, 255 as u8]);
        start_point = start_point + step;
    }

    let mut hour_radius: f32 = 100.0;
    let mut ang = 0.33333 * std::f32::consts::PI;
    let mut rad = 0.0;
    while rad <= hour_radius{
        let hour_x = circle_x + rad * ang.cos();
        let hour_y = circle_y - rad * ang.sin();
        *imgbuf.get_pixel_mut(hour_x as u32, hour_y as u32) = image::Rgb([255 as u8, 255 as u8, 255 as u8]);
        rad = rad + step;
    }

    hour_radius = 75.0;
    ang = 0.7250 * std::f32::consts::PI;
    rad = 0.0;
    while rad <= hour_radius{
        let hour_x = circle_x + rad * ang.cos();
        let hour_y = circle_y - rad * ang.sin();
        *imgbuf.get_pixel_mut(hour_x as u32, hour_y as u32) = image::Rgb([255 as u8, 255 as u8, 255 as u8]);
        rad = rad + step;
    }

    hour_radius = 50.0;
    ang = 1.561 * std::f32::consts::PI;
    rad = 0.0;
    while rad <= hour_radius{
        let hour_x = circle_x + rad * ang.cos();
        let hour_y = circle_y - rad * ang.sin();
        *imgbuf.get_pixel_mut(hour_x as u32, hour_y as u32) = image::Rgb([255 as u8, 255 as u8, 255 as u8]);
        rad = rad + step;
    }


    match imgbuf.save("_output_/output.png"){
        Ok(_) => println!("[*] Successfully saved to '_output_/output.png'"),
        Err(e) => println!("[!] Failed to save into file '_output_/output.png' with error {:?}", e)
    };
}
