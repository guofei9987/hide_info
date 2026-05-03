use anyhow::Result;
use image::{ImageBuffer, Luma, Rgb, Rgba};
use std::io::Cursor;

// 未测试
pub(crate) fn png2rgba(data_bytes: &[u8]) -> Result<Vec<Vec<[u8; 4]>>> {
    let img_reader = Cursor::new(data_bytes);
    let image = image::load(img_reader, image::ImageFormat::Png)?.to_rgba8();

    let (width, height) = image.dimensions();
    let (width, height) = (width as usize, height as usize);

    let mut pixels_rgba: Vec<Vec<[u8; 4]>> = vec![vec![[0; 4]; width]; height];
    for pix_y in 0..height {
        for pix_x in 0..width {
            let p = image.get_pixel(pix_x as u32, pix_y as u32);
            pixels_rgba[pix_y][pix_x] = [p[0], p[1], p[2], p[3]];
        }
    }
    Ok(pixels_rgba)
}

// 未测试
pub(crate) fn rgba2png(pixels: &Vec<Vec<[u8; 4]>>) -> Result<Vec<u8>> {
    let (width, height) = (pixels[0].len(), pixels.len());
    let mut image = ImageBuffer::new(width as u32, height as u32);

    for pix_y in 0..height {
        for pix_x in 0..width {
            let p = pixels[pix_y][pix_x];
            let new_pixel = Rgba(p);
            image.put_pixel(pix_x as u32, pix_y as u32, new_pixel);
        }
    }

    let mut bytes: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(&mut bytes);
    image.write_to(&mut cursor, image::ImageFormat::Png)?;

    Ok(bytes)
}

// 未测试
pub(crate) fn png2gray(data_bytes: &[u8]) -> Result<Vec<Vec<u8>>> {
    let img_reader = Cursor::new(data_bytes);
    let image = image::load(img_reader, image::ImageFormat::Png)?.to_luma8();

    let (width, height) = image.dimensions();
    let (width, height) = (width as usize, height as usize);

    let mut pixels_rgba: Vec<Vec<u8>> = vec![vec![0; width]; height];
    for pix_y in 0..height {
        for pix_x in 0..width {
            let p = image.get_pixel(pix_x as u32, pix_y as u32);
            pixels_rgba[pix_y][pix_x] = p[0];
        }
    }
    Ok(pixels_rgba)
}

// 未测试
pub(crate) fn gray2png(pixels: &Vec<Vec<u8>>) -> Result<Vec<u8>> {
    let (width, height) = (pixels[0].len(), pixels.len());
    let mut image = ImageBuffer::new(width as u32, height as u32);

    for pix_y in 0..height {
        for pix_x in 0..width {
            let p = pixels[pix_y][pix_x];
            let new_pixel = Luma([p]);
            image.put_pixel(pix_x as u32, pix_y as u32, new_pixel);
        }
    }
    let mut bytes: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(&mut bytes);

    image.write_to(&mut cursor, image::ImageFormat::Png)?;

    Ok(bytes)
}

pub(crate) fn png2rgb(data_bytes: &[u8]) -> Result<Vec<Vec<[u8; 3]>>> {
    let img_reader = Cursor::new(data_bytes);
    let img = image::load(img_reader, image::ImageFormat::Png)?.to_rgb8();
    let (width, height) = img.dimensions();
    let (width, height) = (width as usize, height as usize);

    let mut rgb = vec![vec![[0; 3]; width]; height];
    for pix_y in 0..height {
        for pix_x in 0..width {
            let p = img.get_pixel(pix_x as u32, pix_y as u32);
            rgb[pix_y][pix_x] = [p[0], p[1], p[2]];
        }
    }
    Ok(rgb)
}


pub(crate) fn rgb2png(pixels: &Vec<Vec<[u8; 3]>>) -> Result<Vec<u8>> {
    let (width, height) = (pixels[0].len(), pixels.len());
    let mut image = ImageBuffer::new(width as u32, height as u32);

    for pix_y in 0..height {
        for pix_x in 0..width {
            let p = pixels[pix_y][pix_x];
            let new_pixel = Rgb(p);
            image.put_pixel(pix_x as u32, pix_y as u32, new_pixel);
        }
    }
    let mut bytes: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(&mut bytes);
    image.write_to(&mut cursor, image::ImageFormat::Png)?;

    Ok(bytes)
}
