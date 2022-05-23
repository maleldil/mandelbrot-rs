
use png::Encoder;
use std::fs::File;
use std::io::BufWriter;
use std::io::Error;
use std::u8;

pub struct Image {
    file_path: String,
    width: usize,
    height: usize,
}

impl Image {
    pub fn new(file_path: String, width: usize, height: usize) -> Self {
        Image {
            file_path,
            width,
            height,
        }
    }

    pub fn draw_image(&self, pixels: &[u8]) -> Result<(), Error> {
        let file_buffer = File::create(self.file_path.as_str())?;
        let buffered_writer = BufWriter::with_capacity(self.width * self.height, file_buffer);
        let mut encoder = Encoder::new(buffered_writer, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::Grayscale);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(pixels).unwrap();
        Ok(())
    }
}
