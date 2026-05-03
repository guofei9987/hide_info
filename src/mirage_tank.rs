use anyhow::{anyhow, Result};
use image::{imageops::FilterType, ImageBuffer, Luma};

use crate::utils::png::rgba2png;

pub fn mirage_tank_pixels(
    img1_rgb: &[Vec<[u8; 3]>],
    img2_gray: &[Vec<u8>],
    a: f32,
    b: Option<f32>,
) -> Result<Vec<Vec<[u8; 4]>>> {
    let height = img1_rgb.len();
    let width = img1_rgb[0].len();
    if img2_gray.len() != height || img2_gray[0].len() != width {
        return Err(anyhow!("img1_rgb and img2_gray must have the same dimensions"));
    }

    let mut total = 0f64;
    for row in img1_rgb {
        for pixel in row {
            total += (pixel[0] as f64 + pixel[1] as f64 + pixel[2] as f64) / 3.0;
        }
    }
    let mean = (total / (width * height) as f64) as f32;
    let b = b.unwrap_or_else(|| (mean - 255.0 * a).max(10.0));

    let mut result: Vec<Vec<[u8; 4]>> = vec![vec![[0; 4]; width]; height];
    for y in 0..height {
        for x in 0..width {
            let rgb = img1_rgb[y][x];
            let avg = (rgb[0] as f32 + rgb[1] as f32 + rgb[2] as f32) / 3.0;
            let grey = img2_gray[y][x] as f32;
            let grey_adjusted = a * grey + b;

            let alpha_value = (255.0 - avg + grey_adjusted).clamp(0.0, 255.0);
            let alpha = alpha_value.round() as u8;

            let rgb_out = if alpha_value <= 0.0 {
                [0.0, 0.0, 0.0]
            } else {
                let alpha_ratio = alpha_value / 255.0;
                let base = 255.0 - alpha_value;
                [
                    ((rgb[0] as f32 - base) / alpha_ratio).clamp(0.0, 255.0),
                    ((rgb[1] as f32 - base) / alpha_ratio).clamp(0.0, 255.0),
                    ((rgb[2] as f32 - base) / alpha_ratio).clamp(0.0, 255.0),
                ]
            };

            result[y][x] = [rgb_out[0] as u8, rgb_out[1] as u8, rgb_out[2] as u8, alpha];
        }
    }

    Ok(result)
}

pub fn mirage_tank_from_bytes(
    img1_bytes: &[u8],
    img2_bytes: &[u8],
    a: f32,
    b: Option<f32>,
) -> Result<Vec<u8>> {
    let img1 = image::load_from_memory(img1_bytes)?.to_rgb8();
    let img2 = image::load_from_memory(img2_bytes)?.to_luma8();

    let (width1, height1) = img1.dimensions();
    let mut img1_rgb: Vec<Vec<[u8; 3]>> = vec![vec![[0u8; 3]; width1 as usize]; height1 as usize];
    for y in 0..height1 {
        for x in 0..width1 {
            let pixel = img1.get_pixel(x, y);
            img1_rgb[y as usize][x as usize] = [pixel[0], pixel[1], pixel[2]];
        }
    }

    let (width2, height2) = img2.dimensions();
    let mut gray_image = ImageBuffer::new(width2, height2);
    for y in 0..height2 {
        for x in 0..width2 {
            gray_image.put_pixel(x, y, Luma([img2.get_pixel(x, y)[0]]));
        }
    }

    let resized_gray = if width2 == width1 && height2 == height1 {
        gray_image
    } else {
        image::imageops::resize(&gray_image, width1, height1, FilterType::Triangle)
    };

    let mut img2_gray: Vec<Vec<u8>> = vec![vec![0u8; width1 as usize]; height1 as usize];
    for y in 0..height1 {
        for x in 0..width1 {
            img2_gray[y as usize][x as usize] = resized_gray.get_pixel(x, y)[0];
        }
    }

    let rgba_pixels = mirage_tank_pixels(&img1_rgb, &img2_gray, a, b)?;
    rgba2png(&rgba_pixels)
}
