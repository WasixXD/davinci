mod encoder;
mod decoder;
use encoder::{VideoOptions};
use std::thread;

use std::sync::{Arc, Mutex};



const BYTES_PER_FRAME: f32 = 1080f32 * 1920f32 * 3f32;
const N_THREADS: i32 = 5;


fn main() {
    // let file_path = String::from("hello.txt");
    // let content = encoder::read_file_hex(file_path);
    //
    //
    //
    // // calculate how many frames will be
    // let frames_in_total: i32 = ((content.clone().unwrap().len() as f32 / 2f32) / BYTES_PER_FRAME).ceil() as i32;
    //
    //
    // let encoded = encoder::encode_to_video(content.unwrap(), frames_in_total);
    //
    //
    // match encoded {
    //     VideoOptions::Frame(f) => {
    //         let handle = thread::spawn(move || {
    //             encoder::create_frame(f, 1)
    //         });
    //         handle.join().unwrap();
    //     },
    //     VideoOptions::Frames(fs) => {
    //         let mut threads = vec![];
    //         let mutex_copy = Arc::new(Mutex::new(fs));
    //         for _ in 0..N_THREADS {
    //             let f_mutex = Arc::clone(&mutex_copy);
    //
    //             let thread_handle = thread::spawn(move || {
    //                 loop {
    //                     let vetor;
    //                     let i;
    //                     {
    //                         let mut guard = f_mutex.lock().unwrap();
    //                         i = guard.len() as i32;
    //
    //                         if let Some(v) = guard.pop() {
    //                             vetor = v;
    //                         } else {
    //                             break;
    //                         }
    //                     }
    //
    //                     encoder::create_frame(vetor, i);
    //                 }
    //
    //             
    //             });
    //
    //             threads.push(thread_handle);
    //
    //         }
    //
    //         for n in threads {
    //             n.join().unwrap();
    //         }
    //                    
    //
    //
    //     },
    //     VideoOptions::Failed => eprintln!("ERROR: Could not transform into frames")
    // }
    //
    //
    // let output_path = String::from("output.mp4");
    // let _ = encoder::create_video(output_path);





    // let video_path = "output.mp4".to_string();
    // decoder::disassemble_video(video_path);
    // decoder::decode_to_file();
 


}

