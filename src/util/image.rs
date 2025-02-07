use std::io::Cursor;

use fast_image_resize::{CropBox, FilterType, ResizeAlg, ResizeOptions, Resizer, SrcCropping};
use image::{DynamicImage, ImageFormat, ImageReader};

use crate::error::{Error, Result};

pub struct Image {
    format: ImageFormat,
    src: DynamicImage,
    width: u32,
    height: u32,
}

impl TryFrom<Vec<u8>> for Image {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self> {
        let reader = ImageReader::new(Cursor::new(value)).with_guessed_format()?;
        let format = reader
            .format()
            .ok_or(Error::Parse("image format could not be guessed."))?;

        let src = reader.decode()?;
        let width = src.width();
        let height = src.height();

        Ok(Image {
            format,
            src,
            width,
            height,
        })
    }
}

impl TryInto<Vec<u8>> for &Image {
    type Error = Error;

    fn try_into(self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        self.src
            .write_to(&mut Cursor::new(&mut bytes), self.format)?;
        Ok(bytes)
    }
}

impl Image {
    pub fn format(&self) -> ImageFormat {
        self.format
    }

    pub fn resize(self, width: u32, height: u32) -> Result<Self> {
        let mut src = DynamicImage::new(width, height, self.src.color());

        let mut resizer = Resizer::new();
        let resize_options = ResizeOptions {
            algorithm: ResizeAlg::Convolution(FilterType::Lanczos3),
            cropping: SrcCropping::Crop(CropBox::fit_src_into_dst_size(
                self.width,
                self.height,
                width,
                height,
                Some((0.5, 0.5)),
            )),
            ..Default::default()
        };

        resizer.resize(&self.src, &mut src, Some(&resize_options))?;

        Ok(Image {
            format: self.format,
            src,
            width,
            height,
        })
    }
}
