use image::Rgb;
use image::RgbImage;
use std::{fs::File, io};

/// Gray-scale (u8) image
pub struct GrayScaleImage {
    // pub data: SmallVec<[u8; MAX_SIZE]>,
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl GrayScaleImage {
    pub fn with_dimensions(width: u32, height: u32) -> Self {
        let pixels = (width * height) as usize;

        Self {
            data: Vec::with_capacity(pixels),
            width,
            height,
        }
    }

    fn read_pixel(&self, x: u32, y: u32) -> u8 {
        let index = x + y * self.width;
        self.data[index as usize]
    }

    pub fn save_file(&self, filename: &str) -> Result<(), image::ImageError> {
        let mut img = RgbImage::new(self.width, self.height);

        for x in 0..self.width {
            for y in 0..self.height {
                let c = self.read_pixel(x, y);
                img.put_pixel(x, y, Rgb([c, c, c]));
            }
        }

        img.save(filename)
    }
}

/// RGB (u8,u8,u8) image
pub struct MyRgbImage {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl MyRgbImage {
    pub fn load_rgb_image(path: &str) -> io::Result<MyRgbImage> {
        use png::ColorType::*;
        let mut decoder = png::Decoder::new(File::open(path)?);
        decoder.set_transformations(png::Transformations::normalize_to_color8());
        let mut reader = decoder.read_info()?;
        let mut img_data = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut img_data)?;

        // Let's just handle RGB(u8, u8, u8) for now
        let data = match info.color_type {
            Rgb => img_data,
            _ => unreachable!("uncovered color type"),
        };

        Ok(MyRgbImage {
            data,
            width: info.width,
            height: info.height,
        })
    }

    pub fn to_gray_scale_image(&self) -> GrayScaleImage {
        let mut image = GrayScaleImage::with_dimensions(self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                let index = (x * 3 + y * 3 * self.width) as usize;

                let r = self.data[index];
                let g = self.data[index + 1];
                let b = self.data[index + 2];

                // \mathbf{grayscale = 0.3 * R + 0.59 * G + 0.11 * B}
                let v = (0.3 * (r as f32) + 0.59 * (g as f32) + 0.11 * (b as f32)) as u8;

                image.data.push(v);
            }
        }

        image
    }
}
