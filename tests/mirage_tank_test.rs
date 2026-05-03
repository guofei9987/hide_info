use hide_info::mirage_tank::{mirage_tank_from_bytes, mirage_tank_pixels};
use image::{ImageBuffer, ImageFormat, Luma, Rgb};
use std::io::Cursor;
use std::fs;
use anyhow::Result;
#[test]
fn test_mirage_tank_from_bytes() -> Result<()> {
    let img1_bytes = fs::read("./files/jpg1.jpg")?;
    let img2_bytes = fs::read("./files/png2.png")?;

    let output_png = mirage_tank_from_bytes(&img1_bytes,
                                            &img2_bytes,
                                            0.5,
                                            None, )?;

    fs::write("./files/output_mirage.png", output_png)?;
    Ok(())
}
