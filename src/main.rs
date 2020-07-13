extern crate image;
use image::{ImageBuffer};

fn main() {
    let image_height: u32 = 256;
    let image_width: u32 = 256;
    let mut imgbuf = ImageBuffer::new(image_width, image_height);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut(){
        let r = (x as f64) / ((image_width - 1) as f64);
        let g = (y as f64) / ((image_height - 1) as f64);
        let b: f64 = 0.25;
        let ir: u8 = (255.999 * r) as u8;
        let ig: u8 = (255.999 * g) as u8;
        let ib: u8 = (255.999 * b) as u8;
        *pixel = image::Rgb([ir, ig, ib]);
    }
    match imgbuf.save("_output_/output.png"){
        Ok(_) => println!("[*] Successfully saved to '_output_/output.png'"),
        Err(e) => println!("[!] Failed to save into file '_output_/output.png' with error {:?}", e)
    };
}
