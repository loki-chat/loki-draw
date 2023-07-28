use std::fs::File;

use loki_draw::font::Font;
use pix::Raster;
use png_pong::{Encoder, PngRaster, Step};

const ROBOTO_FONT: &[u8] = include_bytes!("common/Roboto-Regular.ttf");
fn main() {
    let font = Font::from_data(ROBOTO_FONT);
    let images = font.render("Hello!", 0.0, 0.0, 100.0, [1.0; 3]);
    let mut i = 0;
    for image in images {
        i += 1;
        let mut encoder =
            Encoder::new(File::create(format!("image{i}.png")).unwrap()).into_step_enc();
        let mut data: Vec<u8> = image.data.into();
        for (i, item) in data.iter_mut().enumerate() {
            if i % 4 == 3 {
                *item = 255;
            }
        }
        let raster = PngRaster::Rgba8(Raster::with_u8_buffer(
            image.placement.width,
            image.placement.height,
            data,
        ));
        encoder.encode(&Step { raster, delay: 0 }).unwrap();
    }
}
