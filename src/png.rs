use image::{Luma};
use qrcode::QrCode;
// use base64;
use std::io::Cursor;

pub fn run() {
  let data = "https://example.com";
  let code = QrCode::new(data.as_bytes()).unwrap();
  let img = code.render::<Luma<u8>>()
                .min_dimensions(5000, 5000)
                .build();
  let mut bytes: Vec<u8> = Vec::new();
  img.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png).unwrap();
  img.save("./qrcode.png").unwrap();
}