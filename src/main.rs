extern crate image;
mod Vec3;
mod Ray;
use image::{ImageBuffer};
use Vec3::vec3d;
use Ray::ray;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
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

    let line_cnt: i32 = 30;
    let angle: f32 = 2.0 * std::f32::consts::PI;
    let angle_step: f32 = angle / line_cnt as f32;
    let mut start: f32 = 0.0;
    let mut radius_set: [i32; 35] = [120; 35];
    let mut radius_start: [i32; 35] = [20; 35];
    let mut cnt = 0;
    let step = 0.001;
    while start < angle{
        let mut radius: f32 = radius_set[cnt] as f32 - rng.gen_range(5.0, 10.0);
        let mut rad = radius_start[cnt] as f32 - rng.gen_range(0.0, 10.0);
        radius = rng.gen_range(rad, radius_set[cnt] as f32) / 10.0 + radius / 10.0 * 9.0;
        if cnt > 18 && cnt < 20{
            let mut radius_sub: f32 = rng.gen_range(50.0, 60.0);
            let mut rad_sub = rng.gen_range(10.0, 20.0);
            let angle_start = start + rng.gen_range(-1.5, 1.5);
            while rad_sub <= radius_sub{
                let hour_x = circle_x + rad_sub * angle_start.cos();
                let hour_y = circle_y - rad_sub * angle_start.sin();
                *imgbuf.get_pixel_mut(hour_x as u32, hour_y as u32) = image::Rgb([255 as u8, 255 as u8, 255 as u8]);
                rad_sub = rad_sub + step;
            }

            let mut radius_sub: f32 = rng.gen_range(50.0, 60.0);
            let mut rad_sub = rng.gen_range(10.0, 20.0);
            let angle_start = start + rng.gen_range(-1.5, 1.5);
            while rad_sub <= radius_sub{
                let hour_x = circle_x + rad_sub * angle_start.cos();
                let hour_y = circle_y - rad_sub * angle_start.sin();
                *imgbuf.get_pixel_mut(hour_x as u32, hour_y as u32) = image::Rgb([255 as u8, 255 as u8, 255 as u8]);
                rad_sub = rad_sub + step;
            }
            radius = rng.gen_range(rad, radius_set[cnt] as f32) / 10.0 + radius / 10.0 * 8.0;
        }
        
        while rad <= radius{
            let hour_x = circle_x + rad * start.cos();
            let hour_y = circle_y - rad * start.sin();
            *imgbuf.get_pixel_mut(hour_x as u32, hour_y as u32) = image::Rgb([255 as u8, 255 as u8, 255 as u8]);
            rad = rad + step;
        }
        cnt = cnt + 1;
        start = start + angle_step;
    }
    cnt = 0;
    start = 0.0;
    while start < angle{
        let mut radius: f32 = rng.gen_range(70.0, 90.0);
        let mut rad = rng.gen_range(20.0, 35.0);
        radius = rng.gen_range(rad, radius_set[cnt] as f32) / 10.0 + radius / 10.0 * 9.0;
        let angle = start + rng.gen_range(-1.5, 1.5);
        while rad <= radius{
            let hour_x = circle_x + rad * (angle).cos();
            let hour_y = circle_y - rad * (angle).sin();
            *imgbuf.get_pixel_mut(hour_x as u32, hour_y as u32) = image::Rgb([255 as u8, 255 as u8, 255 as u8]);
            rad = rad + step;
        }
        cnt = cnt + 1;
        start = start + angle_step;
    }


    cnt = 0;
    start = 0.0;
    while start < angle{
        let mut radius: f32 = rng.gen_range(100.0, 120.0);
        let mut rad = rng.gen_range(20.0, 35.0);
        radius = rng.gen_range(rad, radius_set[cnt] as f32) / 10.0 + radius / 10.0 * 9.0;
        let angle = start + rng.gen_range(-1.5, 1.5);
        while rad <= radius{
            let hour_x = circle_x + rad * (angle).cos();
            let hour_y = circle_y - rad * (angle).sin();
            *imgbuf.get_pixel_mut(hour_x as u32, hour_y as u32) = image::Rgb([255 as u8, 255 as u8, 255 as u8]);
            rad = rad + step;
        }
        cnt = cnt + 1;
        start = start + angle_step;
    }

    match imgbuf.save("_output_/output.png"){
        Ok(_) => println!("[*] Successfully saved to '_output_/output.png'"),
        Err(e) => println!("[!] Failed to save into file '_output_/output.png' with error {:?}", e)
    };
}
