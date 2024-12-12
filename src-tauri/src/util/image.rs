use std::io::Cursor;

use image::imageops::FilterType;
use image::{DynamicImage, ImageFormat, ImageReader};

use crate::Result;

pub struct Image {
    format: ImageFormat,
    image_src: DynamicImage,
    width: usize,
    height: usize,
}

impl TryFrom<Vec<u8>> for Image {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        let reader = ImageReader::new(Cursor::new(value)).with_guessed_format()?;
        let format = reader
            .format()
            .ok_or("image format could not be guessed.")?;

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
    type Error = Box<dyn std::error::Error>;

    fn try_into(self) -> std::result::Result<Vec<u8>, Self::Error> {
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
