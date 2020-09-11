use brilliance::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;

struct Planet {
	col: usize,
	radius_m: f64,
	texture: Arc<dyn Texture>,
}

fn main() -> Result<()> {
	// change to false 3D view.
	let topview = false;

	// read planet positions from file
	let all_positions = parse("output.csv")?;

	// Astronomic unit. All positions scaled by this to keep numbers reasonable.
	let au = 150e9;

	// Scale planet sizes to make better visible.
	let scale_planets = 0.7;

	// Planet properties
	let planet_propts = [
		Planet {
			col: 1,
			radius_m: 2_439e3,
			texture: tex("mercury.jpg", ORANGE),
		},
		Planet {
			col: 2,
			radius_m: 6_051e3,
			texture: tex("venus.jpg", ORANGE),
		},
		Planet {
			col: 3,
			radius_m: 6371e3,
			texture: tex("earth.jpg", BLUE),
		},
		Planet {
			col: 4,
			radius_m: 1738e3, // TODO: why does the moon look so small??
			texture: tex("moon.jpg", GRAY),
		},
		Planet {
			col: 5,
			radius_m: 3_396e3,
			texture: tex("mars.jpg", RED),
		},
		Planet {
			col: 6,
			radius_m: 69_911e3,
			texture: tex("jupiter.jpg", RED),
		},
	];

	let backdrop: Arc<dyn Texture> = if topview { Arc::new(BLACK) } else { tex("milkyway.jpg", BLACK) };

	for (i, positions) in all_positions.iter().enumerate() {
		// Celestial body models
		let mut objects = Vec::new();

		// Add the sun. Size scaled.
		let sun_scale = 20.0;
		let diam_sun = 2.0 * 695_700e3 * sun_scale / au;
		objects.push(DynObj::new(Sphere::new(Point(0., 0., 0.), diam_sun).paint(flat(YELLOW))));

		// Add the planets
		for (i, p) in planet_propts.iter().enumerate() {
			let diam = if topview {
				10000e3 * scale_planets / au
			} else {
				2.0 * p.radius_m * scale_planets / au
			};
			let pos = positions[p.col] / au;
			//dbg!(p.col, pos, diam);
			objects.push(DynObj::new(Sphere::new(pos, diam).paint(flat(p.texture.clone()))));
		}

		//let sun = DynLight::new(PointLight::new(Point(0.0, 0.1, 0.0), Color::new(1.0, 0.95, 0.70).ev(5.5)));

		let s = Scene {
			max_recursion_depth: 1,
			max_iter: 1,
			ambient: WHITE.ev(-4.0),
			background: flat(UVMapped::sphere(backdrop.clone())),
			objects: vec![DynObj::new(QTree::new(objects))],
			lights: vec![], // TODO
		};

		let c = if topview {
			Camera::pinhole(30.0 * DEG)
				.at(Point(0.0, 25.0, 0.0))
				.look_at2(Point(0., 0., 0.), Vector(0.0, 0.0, 1.0))
		} else {
			let p_met = positions[7] / au;
			let p_met_next = all_positions[i + 1][7] / au;
			//let p_earth = positions[3] / au;
			//let p_moon = positions[4] / au;
			Camera::pinhole(60.0 * DEG).at(p_met).look_at(p_met_next)
		};

		let v = View {
			camera: c,
			width: 1920 / 2,
			height: 1080 / 2,
		};

		let num_cpu = 8;
		let quality = 98;
		println!("rendering: {}", i);
		let img = render(&s, &v, num_cpu);
		save_jpg(&img, &format!("out/{:04}.jpg", i), quality)?;
	}

	Ok(())
}

fn tex(fname: &str, fallback: Color) -> Arc<dyn Texture> {
	match load(fname) {
		Ok(tex) => Arc::new(tex),
		Err(e) => {
			eprintln!("failed to load {}: {}. falling back to solid color {}", fname, e.to_string(), fallback);
			Arc::new(fallback)
		}
	}
}

fn parse(fname: &str) -> Result<Vec<Vec<Point>>> {
	let fields = parse_fields(fname)?;
	let mut result = Vec::new();
	for fields in fields {
		result.push(vectorize(fields)?)
	}
	Ok(result)
}

fn vectorize(fields: Vec<f64>) -> Result<Vec<Point>> {
	let mut result = Vec::with_capacity(fields.len());

	if fields.len() % 3 != 0 {
		return error(format!("fiels not a multiple of 3"));
	}

	let mut it = fields.iter();
	for _ in 0..(fields.len() / 3) {
		let x = *it.next().unwrap();
		let y = *it.next().unwrap();
		let z = *it.next().unwrap();
		result.push(Point::new(x, z, y));
	}

	Ok(result)
}

fn parse_fields(fname: &str) -> Result<Vec<Vec<f64>>> {
	let mut result = Vec::new();

	let f = File::open(fname)?;
	let reader = BufReader::new(f);
	for line in reader.lines().skip(1) {
		let line = line?;
		let fields = line.split(",");
		let mut values = Vec::new();
		for field in fields {
			if field == "" {
				break;
			}
			let v: f64 = parse_f64(field.trim())?;
			values.push(v);
		}
		result.push(values);
	}

	Ok(result)
}

fn parse_f64(s: &str) -> Result<f64> {
	match s.parse() {
		Ok(v) => Ok(v),
		Err(e) => error(format!("parse {}: {}", s, e.to_string())),
	}
}
