use std::{fs};
use imageproc::drawing::{draw_text_mut};
use rusttype::{Font, Scale};
use std::path;

pub struct Ticket<'a> {
    image_path: &'a path::Path,
    text: &'a str,
}

impl<'a> Ticket<'a> {
    pub fn new(image_path: &'a path::Path, text: &'a str,) -> Self {
        Ticket {
            image_path,
            text,
        }
    }

    pub fn print(self, output: &str) -> Result<path::PathBuf, anyhow::Error>{
        let _ = fs::copy(self.image_path, output)?;
        let ticket_image = &path::Path::new(output);
        let mut image = image::open(ticket_image)?;

        // esto es una porquería, cambiarlo para que use fontkit
        let font = Vec::from(include_bytes!("../DejaVuSans.ttf") as &[u8]);
        let font = Font::try_from_vec(font).ok_or_else(|| {todo!()}).unwrap();

        let height = 80.0;
        let scale = Scale {
            x: height * 1.0,
            y: height,
        };

        draw_text_mut(&mut image, image::Rgba([255u8, 255u8, 255u8, 1u8]), 862, 286, scale, &font, self.text);

        let _ = image.save(ticket_image)?;

        Ok(ticket_image.to_path_buf())
    }
}