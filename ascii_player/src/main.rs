use image::io::Reader as ImageReader;
use image::DynamicImage;

const ASCII_CHARS: &[u8] = b" ._-,=+:;cba!?#@";


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open("Test2.png")?.decode()?;
    println!("Image loaded successfully!");

    let gray_image = img.to_luma8();
    use image::imageops::{contrast, brighten};

    // Adjust brightness
    let gray_image = brighten(&gray_image, 33); // Increase brightness by 10
    //let high_contrast_image = contrast(&gray_image, 0.1);

    use image::imageops::FilterType;
    let resized_image = resize_with_aspect_ratio(&gray_image, 0.25);

    let ascii_art = image_to_ascii(&resized_image);


    for line in &ascii_art {
        println!("{}", line);
    }
    Ok(())
}

fn pixel_to_ascii(value: u8) -> char {
    let index = (value as usize * (ASCII_CHARS.len() - 1)) / 255;
    ASCII_CHARS[index] as char
}

fn image_to_ascii(img: &image::GrayImage) -> Vec<String> {
    img.rows()
        .map(|row| {
            row.map(|pixel| pixel_to_ascii(pixel[0]))
                .collect()
        })
        .collect()
}

use image::{imageops::FilterType, GrayImage};
fn resize_with_aspect_ratio(img: &GrayImage, scale_factor: f32) -> GrayImage {
    let aspect_ratio_correction = 2.0; // Adjust this to match your terminal's font ratio
    let (original_width, original_height) = img.dimensions();
    let new_width = (original_width as f32 * scale_factor) as u32;
    let new_height = (original_height as f32 / aspect_ratio_correction * scale_factor) as u32;

    image::imageops::resize(img, new_width, new_height, FilterType::Nearest)
}

// use ffmpeg_next as ffmpeg;
// use image::{GrayImage, Luma, imageops::resize};
// use std::{error::Error, process::Command, thread::sleep, time::Duration};
// 
// const ASCII_CHARS: &[u8] = b" ._-,=+:;cba!?#@";
// const FRAME_DELAY: u64 = 100; // Default frame delay in milliseconds
// 
// fn main() -> Result<(), Box<dyn Error>> {
//     // Initialize FFmpeg
//     ffmpeg::init()?;
// 
//     // Extract frames using FFmpeg
//     let input_file = "example.mp4"; // Replace with your video file path
//     extract_frames(input_file)?;
// 
//     // Iterate through extracted frames
//     let frames_dir = "frames"; // Directory containing the extracted frames
//     for frame_path in std::fs::read_dir(frames_dir)? {
//         let frame_path = frame_path?.path();
// 
//         // Load the frame as a grayscale image
//         let gray_frame = image::open(&frame_path)?.to_luma8();
// 
//         // Resize and convert to ASCII
//         let resized_frame = resize_with_aspect_ratio(&gray_frame, 0.2); // Scale factor 20%
//         let ascii_art = image_to_ascii(&resized_frame);
// 
//         // Display the ASCII art
//         print_ascii_frame(&ascii_art);
//         sleep(Duration::from_millis(FRAME_DELAY)); // Simulate frame delay
//     }
// 
//     Ok(())
// }
// 
// fn extract_frames(input_file: &str) -> Result<(), Box<dyn Error>> {
//     // Use FFmpeg CLI to extract frames from the video
//     std::fs::create_dir_all("frames")?;
//     Command::new("ffmpeg")
//         .arg("-i")
//         .arg(input_file)
//         .arg("frames/frame_%04d.png")
//         .output()?;
//     Ok(())
// }
// 
// fn resize_with_aspect_ratio(img: &GrayImage, scale_factor: f32) -> GrayImage {
//     let aspect_ratio_correction = 2.0; // Adjust for terminal font
//     let (original_width, original_height) = img.dimensions();
//     let new_width = (original_width as f32 * scale_factor) as u32;
//     let new_height = (original_height as f32 / aspect_ratio_correction * scale_factor) as u32;
// 
//     resize(img, new_width, new_height, image::imageops::FilterType::Nearest)
// }
// 
// fn image_to_ascii(img: &GrayImage) -> Vec<String> {
//     img.rows()
//         .map(|row| {
//             row.map(|pixel| pixel_to_ascii(pixel[0]))
//                 .collect()
//         })
//         .collect()
// }
// 
// fn pixel_to_ascii(value: u8) -> char {
//     let index = (value as usize * (ASCII_CHARS.len() - 1)) / 255;
//     ASCII_CHARS[index] as char
// }
// 
// fn print_ascii_frame(ascii_art: &[String]) {
//     for line in ascii_art {
//         println!("{}", line);
//     }
// }