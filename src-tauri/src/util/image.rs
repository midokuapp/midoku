use std::io::Cursor;

use image::imageops::FilterType;
use image::{DynamicImage, ImageFormat, ImageReader};

use crate::error::Error;
use crate::Result;

pub struct Image {
    format: ImageFormat,
    image_src: DynamicImage,
    width: usize,
    height: usize,
}

impl TryFrom<Vec<u8>> for Image {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self> {
        let reader = ImageReader::new(Cursor::new(value)).with_guessed_format()?;
        let format = reader
            .format()
            .ok_or(Error::Parse("image format could not be guessed.".into()))?;

        let image_src = reader.decode()?;
        let width = image_src.width() as usize;
        let height = image_src.height() as usize;

        Ok(Image {
            format,
            image_src,
            width,
            height,
        })
    }
}

impl TryInto<Vec<u8>> for &Image {
    type Error = Error;

    fn try_into(self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        self.image_src
            .write_to(&mut Cursor::new(&mut bytes), self.format)?;
        Ok(bytes)
    }
}

impl Image {
    pub fn format(&self) -> ImageFormat {
        self.format
    }

    pub fn resize(mut self, width: usize, height: usize) -> Result<Self> {
        self.image_src =
            self.image_src
                .resize_to_fill(width as u32, height as u32, FilterType::Triangle);
        self.width = width;
        self.height = height;

        Ok(self)
    }
}
