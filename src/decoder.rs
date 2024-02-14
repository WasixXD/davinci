
use crate::encoder;
use std::process::Command;
use std::fs;
use image::{GenericImageView, DynamicImage, Rgba};
use hex;


const DIS_FOLDER: &str = "./dis/";





pub fn disassemble_video(video_path: String) {
    let bash_command = "./bash/disassemble.sh"; 

    if encoder::ensure_dir_exists(DIS_FOLDER.to_string()) {
        Command::new("bash")
            .arg(bash_command)
            .arg(video_path)
            .output()
            .expect("ERROR: Could not disassemble your video");

        


    }
}


pub fn rgb_to_hex(r: u8 ,g: u8, b: u8) -> String {
    
    format!("{:X}", r as u32 * 256 * 256 + g as u32 * 256 + b as u32)
}

pub fn decode_to_file() {

    let files = fs::read_dir(DIS_FOLDER).unwrap();

    for file in files {

        let file_path = file.unwrap().path();
        let img = image::open(file_path.clone()).unwrap();
        let (width, height) = img.dimensions();


        for y in 0..height {
            for x in 0..width {

                let pixel= img.get_pixel(x, y);

                let (r, g, b) = (pixel[0], pixel[1], pixel[2]);


                let hex_code = rgb_to_hex(r, g, b);
                
                let a = 0x7b6279_u32;
                let b = a as i32;
                let c = format!("{:x}", b);

                println!("{}, {}", b, c);
                return;
                let decoded = hex::decode(hex_code);
                let data = String::from_utf8(decoded.unwrap());
                println!("{:?}", data);

            }
        }
    }
}
