use std::{fs::File, io::BufWriter};

use fantasia::*;
use image::{codecs::png::PngEncoder, ImageEncoder};

const WIDTH: usize = 400;
const HEIGHT: usize = 400;

fn main() {
    let mut renderer = Renderer::new(WIDTH, HEIGHT);
    renderer.clear(Rgba::new(0, 0, 0, 255));

    let a = Point::new(0., 1., 0.);
    let b = Point::new(-1., 0., 0.);
    let c = Point::new(1., 0., 0.);

    renderer.line(Rgba::new(255, 255, 255, 255), a, b);
    renderer.line(Rgba::new(255, 255, 255, 255), b, c);
    renderer.line(Rgba::new(255, 255, 255, 255), c, a);

    let buf = image::RgbaImage::from_raw(WIDTH as u32, HEIGHT as u32, renderer.buffer().to_vec())
        .unwrap();

    // write image to file
    let writer = BufWriter::new(File::create("output.png").unwrap());
    let encoder = PngEncoder::new(writer);

    encoder
        .write_image(&buf, WIDTH as u32, HEIGHT as u32, image::ColorType::Rgba8)
        .unwrap();
}
