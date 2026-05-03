use anyhow::Result;
use crate::utils::cipher::{deserialization, serialization};
use crate::utils::png::{png2rgb, rgb2png};

pub struct HideAsImg;

impl HideAsImg {
    pub fn new() -> Self {
        Self
    }

    pub fn encode(&self, bytes_data: &[u8]) -> Result<Vec<u8>> {
        let data_to_write = serialization(bytes_data);
        let len_data = data_to_write.len();
        let width = ((len_data as f64 / 3.0).sqrt().ceil() as u32).max(1) as usize;
        let mut rgb: Vec<Vec<[u8; 3]>> = vec![vec![[0u8; 3]; width]; width];

        for (i, &byte) in data_to_write.iter().enumerate() {
            rgb[(i / 3) / width][i / 3 % width][i % 3] = byte;
        }
        rgb2png(&rgb)
    }

    pub fn decode(&self, data_bytes: &[u8]) -> Result<Vec<u8>> {
        let rgb = png2rgb(data_bytes)?;
        let (width, height) = (rgb[0].len(), rgb.len());
        let mut lst = Vec::with_capacity(width * width);
        for y in 0..height {
            for x in 0..width {
                lst.extend_from_slice(&rgb[y][x]);
            }
        }
        println!("{:?}", lst);
        Ok(deserialization(&lst))
    }
}
