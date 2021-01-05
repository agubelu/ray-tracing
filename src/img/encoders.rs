use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

extern crate png;

///////////////////////////////////////////////////////////////////////////////

pub fn encode_png(width: usize, height: usize, content: &[u8], out_path: &str) {
    let path = Path::new(out_path);
    let img_file = File::create(path).unwrap();

    let bw = BufWriter::new(img_file);
    let mut png_encoder = png::Encoder::new(bw, width as u32, height as u32);
    png_encoder.set_color(png::ColorType::RGB);
    png_encoder.set_depth(png::BitDepth::Eight);

    let mut img_writer = png_encoder.write_header().unwrap();
    img_writer.write_image_data(content).unwrap();
}
