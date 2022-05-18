use qrcode::{QrCode, render::svg};

pub fn run() {
  println!("hello svg");
  let data = "https://example.com";
  let code = QrCode::new(data.as_bytes()).unwrap();
  let svg = code.render::<svg::Color>().build();
  println!("{:?}", svg)
}
