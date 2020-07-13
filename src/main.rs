use std::fs::File;
use std::io::Write;

fn main() {
    let mut ppm_file_result = match File::create("_output_/output.ppm"){
        Ok(v) => {
            println!("[*] File output.ppm created.");
            v
        }, 
        Err(e) => panic!("[!] Unable to create file: {:?}", e),
    };

    let image_height: i32 = 256;
    let image_width: i32 = 256;
    let image_depth: i32 = 255;
    ppm_file_result.write_all(format!("P3\n{} {}\n{}\n", image_width, image_height, image_depth).as_bytes()).expect("[!] P3 Header write failed.");
    for j in 0..image_height {
        for i in 0..image_width{
            let r = (i as f64) / ((image_width - 1) as f64);
            let g = (j as f64) / ((image_height - 1) as f64);
            let b: f64 = 0.25;
            let ir: i32 = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;
            ppm_file_result.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes()).expect("[!] Write Error");
        }
    }
}
