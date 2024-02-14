use hex::encode;
use std::fs;
use std::path::Path;
use image::{RgbImage, Rgb};
use hex_color::HexColor;
use std::process::Command;


pub const FRAMES_FOLDER: &str = "./frames/";

pub const WIDTH: usize = 1080;
pub const HEIGHT: usize = 1920;


#[derive(Debug, PartialEq)]
pub enum VideoOptions {
    Frame(Vec<Pixel>), 
    Frames(Vec<Vec<Pixel>>),
    Failed
}


#[derive(Debug, Clone, PartialEq)]
pub struct Pixel {
    pub x: usize,
    pub y: usize,
    pub hex_data: String 
}


impl Pixel {
    pub fn format_data(&mut self) {
        while self.hex_data.len() < 6 {
           self.hex_data.push('F');
        }
    }
}



pub fn read_file_hex(file_path: String) -> Result<String, &'static str> {
    let result = fs::read(file_path);
    match result {
        Ok(content) => {
            let hex_string = encode(&content);

            return Ok(hex_string);
        }
        Err(_r) => return Err("ERROR: Could not find file"),
    }
}


pub fn ensure_dir_exists(folder_path: String) -> bool {
    if Path::new(&folder_path).exists() {
        return true;
    }

    let result = fs::create_dir(folder_path);


    match result {
        Ok(_) => return true,
        Err(_) => return false,
    }
    
}


pub fn encode_to_video(hex_string: String, nframes: i32) -> VideoOptions { 
    if ensure_dir_exists(FRAMES_FOLDER.to_string()) {
        let mut frame = vec![]; 
        let mut cursor = 0;
        for n in 0..nframes {

            frame.push(vec![]);
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let def_hex;
                    if cursor < hex_string.len() - 6 {
                        def_hex = &hex_string[cursor..cursor + 6];
                        cursor += 6;
                        
                    
                    } else if cursor > hex_string.len() {
                        def_hex = "F";
                    } else {
                        
                        def_hex = &hex_string[cursor..hex_string.len()];
                        cursor += hex_string.len() - cursor;
                    }

                    let mut pixel = Pixel {x: x, y: y, hex_data: String::from(def_hex)};

                    pixel.format_data();


                    frame[n as usize].push(pixel);
                        

                }
            }
        }


        return if nframes == 1 { VideoOptions::Frame(frame.first().unwrap().to_vec() ) } else { VideoOptions::Frames(frame) }
    } else {
        return VideoOptions::Failed;
    } 

}



pub fn create_frame(frame: Vec<Pixel>, n_frame: i32) {
    let mut img = RgbImage::new(1080, 1920);
    for p in &frame {
                    
        let color = HexColor::parse(&format!("#{}", &p.hex_data)).unwrap(); 
        img.put_pixel(p.x.try_into().unwrap() , p.y.try_into().unwrap(), Rgb([color.r, color.g, color.b])); 
    }
    img.save(format!("{}frame{}.bmp", FRAMES_FOLDER, n_frame)).expect("ERROR");
}




pub fn create_video(output_path: String) {

    let bash_command = "./bash/video.sh";

    Command::new("bash")
        .arg(bash_command)
        .arg(output_path)
        .output()
        .expect("ERROR: Could not run bash script");



    //remove and immediately create it
    /* // fs::remove_dir_all(FRAMES_FOLDER).unwrap(); */
    



}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_hex_string() {
        let file_path = String::from("hello.txt");
        let my_hex_string = "48656c6c6f2c20576f726c64210a".to_string();
        let result = read_file_hex(file_path).unwrap();

        assert_eq!(result, my_hex_string)
    }

    #[test]
    fn read_hex_string_with_noe_file() {
        let file_path = String::from("asdf.txt");
        let result = read_file_hex(file_path);
        assert_eq!(result, Err("ERROR: Could not find file"));
    }

    #[test]
    fn check_if_folder_exist() {

        let folder_path = String::from("./frames/");
        let result = ensure_dir_exists(folder_path);
        assert_eq!(result, true);

    }


    #[test]
    fn create_folder_ifnot_exist() {
        let folder_path = String::from("./frames1/");
        let result = ensure_dir_exists(folder_path.clone());
        let expected = Path::new(&folder_path).exists();

        assert_eq!(result, expected);

        let _ = fs::remove_dir(&folder_path);
    }

    #[test]
    fn could_not_create_folder() {
        let folder_path = String::from("./frames1/tmp");
        let result = ensure_dir_exists(folder_path.clone());
        assert_eq!(result, false);
    }


    #[test]
    fn create_single_frame() {
        let my_hex_string = "48656c6c6f2c20576f726c64210a".to_string();
        let my_frame = encode_to_video(my_hex_string, 1);
        let expected_pixel = Pixel {x: 1, y: 1, hex_data: "48656c".to_string() };
        let test_pixel;
        match my_frame {
            VideoOptions::Frame(vec) => {

                test_pixel = vec.first();
                assert_eq!(test_pixel.unwrap().hex_data, expected_pixel.hex_data);
            },
            VideoOptions::Frames(_) | VideoOptions::Failed => todo!(),
        }



    }
}
