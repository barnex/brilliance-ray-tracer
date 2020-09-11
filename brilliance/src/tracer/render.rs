use super::*;
use std::clone::Clone;
use std::cmp::min;
use std::thread::spawn;

pub fn render(scene: &Scene, v: &View, num_threads: u32) -> Image<Color> {
	// tile size in pixels
	const T: u32 = 32;

	let (w, h) = v.dimensions();
	let (nx, ny) = split_tiles((w, h), T);

	let (send_work, recv_work) = mpmc_channel::<Work>();
	for ty in 0..ny {
		for tx in 0..nx {
			let work = Work {
				min: (tx * T, ty * T),
				max: (min(tx * T + T, w), min(ty * T + T, h)),
			};
			send_work.send(work).unwrap();
		}
	}
	drop(send_work); // let workers know no more work is coming

	// Sharing the Scene between worker threads would normally be done with an Arc.
	// However, the natural API is pass a &Scene, not a Scene (or Arc<Scene>).
	//
	// So we promise to the compiler that scene lives long enough here.
	// This is safe because the worker threads exit before this function returns.
	let scene = unsafe { static_cast(scene) };

	let mut handles = Vec::new();
	for i in 0..num_threads {
		let scene = scene.clone();
		let recv_work = recv_work.clone();
		let v = v.clone();
		handles.push(spawn(move || {
			let mut tiles = Vec::new();
			for work in recv_work {
				let rng = TileRng::new((T, T), i);
				let img = render_tile(rng, scene, &v, work.min, work.max);
				tiles.push(Done { min: work.min, img });
			}
			tiles
		}));
	}

	let mut img = Image::new(v.dimensions());
	for h in handles {
		for done in h.join().unwrap() {
			img.draw(done.min, &done.img);
		}
	}

	img
}

// number of `tile` by `tile` pixel tiles needed to cover `w` by `h` image.
pub fn split_tiles((w, h): (u32, u32), tile: u32) -> (u32, u32) {
	(div_up(w, tile), div_up(h, tile))
}

// integer division, rounded up rather than down.
fn div_up(x: u32, y: u32) -> u32 {
	((x - 1) / y) + 1
}

// turn a &T into a &'static T.
unsafe fn static_cast<T>(t: &T) -> &'static T {
	let t: *const T = t;
	&*t
}

fn render_tile(mut rng: TileRng, s: &Scene, v: &View, min: (u32, u32), max: (u32, u32)) -> Image<Color> {
	let (w, h) = v.dimensions();
	let (tw, th) = (max.0 - min.0, max.1 - min.1);

	Image::from_fn((tw, th), |x, y| {
		let mut acc = Color::BLACK;
		for iter in 0..s.max_iter {
			let mut rng = rng.for_pix((x, y), iter);
			let aa = aa(&rng, s);
			let uv = index_to_cam((w, h), ((x + min.0) as f64 + aa.0, (y + min.1) as f64 + aa.1));

			acc += s.image_fn(&mut rng, &v.camera, uv);
		}
		if !acc.is_finite() {
			println!("WARN: got NaN color");
		}
		acc / (s.max_iter as f32)
	})
}

fn aa(rng: &Rng, s: &Scene) -> (f64, f64) {
	if s.max_iter == 1 {
		(0.0, 0.0)
	} else {
		let (aa0, aa1) = rng.quasi_random2();
		(aa0 as f64 - 0.5, aa1 as f64 - 0.5)
	}
}

#[derive(Debug)]
struct Work {
	min: (u32, u32),
	max: (u32, u32),
}

#[derive(Debug)]
struct Done {
	min: (u32, u32),
	img: Image<Color>,
}
