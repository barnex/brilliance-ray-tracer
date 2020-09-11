extern crate image;
use super::*;
use image::jpeg::JpegEncoder;
use std::fs::File;

#[must_use]
pub fn save(img: &Image<Color>, fname: &str) -> Result<()> {
	let (w, h) = img.dimensions();
	match image::save_buffer(fname, &img.raw_rgb(), w as u32, h as u32, image::ColorType::Rgb8) {
		Err(e) => error(format!("save {}: {}", fname, e.to_string())),
		Ok(()) => Ok(()),
	}
}

#[must_use]
pub fn save_jpg(img: &Image<Color>, fname: &str, qual: u8) -> Result<()> {
	let mut f = File::create(fname)?;
	let mut enc = JpegEncoder::new_with_quality(&mut f, qual);
	let (w, h) = img.dimensions();
	match enc.encode(&img.raw_rgb(), w as u32, h as u32, image::ColorType::Rgb8) {
		Err(e) => error(format!("save {}: {}", fname, e.to_string())),
		Ok(()) => Ok(()),
	}
}

pub fn load(fname: &str) -> Result<Image<RGB>> {
	let orig = match image::open(fname) {
		Ok(img) => img,
		Err(e) => return error(format!("load {}: {}", fname, e.to_string())),
	};
	let rgb = orig.into_rgb();

	let img = Image::<RGB>::from_fn(rgb.dimensions(), |x, y| {
		let pix = rgb.get_pixel(x, y);
		[pix[0], pix[1], pix[2]]
	});
	Ok(img)
}
