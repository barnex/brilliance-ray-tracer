pub use super::*;

type Image = super::Image<Color>;

pub fn add_gaussian_bloom(orig: &Image, ampl: f64, sigma: f64) -> Image {
	let (w, h) = orig.dimensions();

	let mut dst1 = Image::new((w, h));
	for iy in 0..(h as usize) {
		for ix in 0..(w as usize) {
			let c = orig[iy][ix];
			add_gauss_x(&mut dst1, (ix, iy), sigma, c * ampl as f32);
		}
	}

	let mut dst2 = Image::new((w, h));
	for iy in 0..(h as usize) {
		for ix in 0..(w as usize) {
			let c = dst1[iy][ix];
			add_gauss_y(&mut dst2, (ix, iy), sigma, c * ampl as f32);
		}
	}

	Image::from_fn(orig.dimensions(), |x, y| orig.at((x, y)) + dst2.at((x, y)))
}

fn add_gauss_x(dst: &mut Image, (x, y): (usize, usize), sigma: f64, intens: Color) {
	let (x, y) = (x as i32, y as i32);
	let (w, h) = dst.dimensions();

	for i in 0..100i32 {
		let x2 = x + i;
		if x2 >= (w as i32) {
			break;
		}
		let v = normal(sigma, i as f64) as f32;
		let c = v * intens;
		if c.max() < 1e-4 {
			break;
		}
		dst[y as usize][x2 as usize] += c;
	}
	for i in 0..100i32 {
		let x2 = x - i;
		if x2 < 0 {
			break;
		}
		let v = normal(sigma, i as f64) as f32;
		let c = v * intens;
		if c.max() < 1e-4 {
			break;
		}
		dst[y as usize][x2 as usize] += c;
	}
}

fn add_gauss_y(dst: &mut Image, (x, y): (usize, usize), sigma: f64, intens: Color) {
	let (x, y) = (x as i32, y as i32);
	let (w, h) = dst.dimensions();

	for i in 0..100i32 {
		let y2 = y + i;
		if y2 >= (h as i32) {
			break;
		}
		let v = normal(sigma, i as f64) as f32;
		let c = v * intens;
		if c.max() < 1e-4 {
			break;
		}
		dst[y2 as usize][x as usize] += c;
	}
	for i in 0..100i32 {
		let y2 = y - i;
		if y2 < 0 {
			break;
		}
		let v = normal(sigma, i as f64) as f32;
		let c = v * intens;
		if c.max() < 1e-4 {
			break;
		}
		dst[y2 as usize][x as usize] += c;
	}
}

/// Standard normal distribution.
///
///     use brilliance::*;
///     assert_eq!(normal(1.0, 0.0), 0.3989422804014327);
///     assert_eq!(normal(1.0, 2.0), 0.05399096651318806);
///     assert_eq!(normal(2.0, 0.0), 0.19947114020071635);
///
pub fn normal(sigma: f64, x: f64) -> f64 {
	let sqrt2pi = f64::sqrt(2.0 * PI);
	1.0 / (sigma * sqrt2pi) * f64::exp(-0.5 * sqr(x / sigma))
}

pub fn add_bleed(orig: &Image, thresh: f32) -> Image {
	let (w, h) = orig.dimensions();
	let mut dst = Image::new((w, h));

	for iy in 0..(h as usize) {
		for ix in 0..(w as usize) {
			let c = orig[iy][ix];
			if c.max() > thresh {
				let overflow = c.apply(|c| re(c - thresh));
				add_bloom(&mut dst, (ix, iy), overflow);
			}
		}
	}

	Image::from_fn(orig.dimensions(), |x, y| orig.at((x, y)) + dst.at((x, y)))
}

fn add_bloom(dst: &mut Image, (x, y): (usize, usize), intens: Color) {
	let (w, h) = dst.dimensions();
	let (x, y) = (x as i32, y as i32);
	let mut intens = intens;

	let ampl = 0.0000002;
	let delta = 10000.0;

	let mut try_add = |x, y, c| {
		if x >= 0 && x < (w as i32) && y >= 0 && y < (h as i32) {
			dst[y as usize][x as usize] += ampl * c;
		}
	};

	for i in 0..200i32 {
		// TODO
		try_add(x, y + i, intens);
		try_add(x, y - i, intens);
		try_add(x + i, y, intens);
		try_add(x - i, y, intens);

		intens = intens.apply(|c| re(c - delta));
		//intens = intens.apply();
		if intens.max() <= 0.1 {
			break;
		}
	}
}
