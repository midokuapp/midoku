use std::io::Cursor;

use image::imageops::FilterType;
use image::ImageReader;

use crate::Result;

pub fn resize(src: Vec<u8>, width: Option<u32>, height: Option<u32>) -> Result<Vec<u8>> {
    if width.is_none() && height.is_none() {
        return Err("either width or height must be specified.".into());
    }

    let reader = ImageReader::new(Cursor::new(src)).with_guessed_format()?;
    let src_format = reader
        .format()
        .ok_or("image format could not be guessed.")?;

    let src_image = reader.decode()?;
    let src_width = src_image.width();
    let src_height = src_image.height();

    // Calculate the width and height of the resized image
    let (dst_width, dst_height) = match (width, height) {
        (Some(width), Some(height)) => (width, height),
        (Some(width), None) => {
            let height = (width as f32 / src_width as f32 * src_height as f32) as u32;
            (width, height)
        }
        (None, Some(height)) => {
            let width = (height as f32 / src_height as f32 * src_width as f32) as u32;
            (width, height)
        }
        _ => unreachable!(),
    };

    let dst_image = src_image.resize_to_fill(dst_width, dst_height, FilterType::Triangle);

    let mut result = Vec::new();
    dst_image.write_to(&mut Cursor::new(&mut result), src_format)?;

    Ok(result)
}
