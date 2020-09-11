use brilliance::*;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;
extern crate structopt;
use structopt::StructOpt;

// Astronomic unit. All positions scaled by this to keep numbers reasonable.
static AU: f64 = 150e9;

static CAM_POS_COL: usize = 0;
static CAM_DIR_COL: usize = 1;

// Command-line options:
/// Render a solar system from planet coordinates in CSV format.
#[derive(StructOpt)]
struct Cli {
	/// CSV input file
	input: String,

	/// Output directory.
	#[structopt(short, long, default_value = "out")]
	output: String,

	/// Image width (pixels).
	#[structopt(short, long, default_value = "960")]
	width: u32,

	/// Image height, (pixels).
	#[structopt(short, long, default_value = "540")]
	height: u32,

	/// Background texture (suggested: https://en.wikipedia.org/wiki/Milky_Way#/media/File:ESO_-_Milky_Way.jpg).
	#[structopt(short, long, default_value = "milkyway.jpg")]
	background: String,

	/// Only output every nth frame, skip others.
	#[structopt(short, long, default_value = "1")]
	every: usize,

	/// Number of samples per pixel, improves quality.
	#[structopt(short, long, default_value = "1")]
	samples: u32,

	/// Activate top view for debugging.
	#[structopt(short, long)]
	topview: bool,

	/// Camera field of view (degreed).
	#[structopt(long, default_value = "60")]
	fov: f64,
}

fn main() -> Result<()> {
	let args = Cli::from_args();
	println!("[ ] ouput directory: {}", args.output);
	println!("[ ] ouput size: {} x {}", args.width, args.height);
	if args.every != 1 {
		println!("[ ] rendering every {}th frame", args.every);
	}
	if let Err(e) = fs::create_dir(&args.output) {
		eprintln!("[?] create directory {}: {}", &args.output, e.to_string())
	}

	// read planet positions from file
	let all_positions = parse_positions_csv(&args.input)?;

	// prepare planet properties and load textures
	let planet_propts = planet_propts();

	// Optionally scale planet sizes to make better visible.
	let scale_planets = 1.0;
	let scale_sun = 1.0;

	// Milky way background
	let backdrop: Arc<dyn Texture> = if args.topview { Arc::new(BLACK) } else { tex(&args.background, BLACK) };

	// Animation: one render per line in the input.
	for (i, positions) in all_positions.iter().enumerate().step_by(args.every) {
		// Celestial body models
		let mut objects = Vec::new();

		// Add the sun.
		let diam_sun = 2.0 * 695_700e3 * scale_sun / AU;
		objects.push(DynObj::new(Sphere::new(Point(0., 0., 0.), diam_sun).paint(flat(YELLOW))));

		// Add the planets
		for p in planet_propts.iter() {
			let diam = if args.topview { 0.1 } else { 2.0 * p.radius_m * scale_planets / AU };
			let pos = positions[p.col] / AU;
			//dbg!(p.col, pos, diam);
			objects.push(DynObj::new(Sphere::new(pos, diam).paint(flat(p.texture.clone()))));
		}

		//let sun = DynLight::new(PointLight::new(Point(0.0, 0.1, 0.0), Color::new(1.0, 0.95, 0.70).ev(5.5)));

		let s = Scene {
			max_recursion_depth: 1,
			max_iter: args.samples,
			ambient: WHITE.ev(-4.0),
			background: flat(UVMapped::sphere(backdrop.clone())),
			objects: vec![DynObj::new(QTree::new(objects))],
			lights: vec![], // TODO
		};

		// Set the camera
		let c = if args.topview {
			Camera::pinhole(30.0 * DEG)
				.at(Point(0.0, 25.0, 0.0))
				.look_at2(Point(0., 0., 0.), Vector(0.0, 0.0, 1.0))
		} else {
			let cam_pos = positions[CAM_POS_COL] / AU;
			let cam_dir = positions[CAM_DIR_COL] / AU;
			Camera::pinhole(args.fov * DEG).at(cam_pos).look_dir(cam_dir, Vector::EY)
		};
		let v = View {
			camera: c,
			width: args.width,
			height: args.height,
		};

		// Render & save
		let num_cpu = 8;
		let quality = 98;
		let img = render(&s, &v, num_cpu);
		let fname = format!("{}{}{:04}.jpg", &args.output, std::path::MAIN_SEPARATOR, i);
		match save_jpg(&img, &fname, quality) {
			Ok(()) => Ok(()),
			Err(e) => error(format!("save {}: {}", &fname, e.to_string())),
		}?;
		println!("[ ] rendered: {}", &fname);
	}

	Ok(())
}

// Return the planets' properties.
fn planet_propts() -> Vec<Planet> {
	vec![
		Planet {
			col: 3,
			radius_m: 2_439e3,
			texture: tex("mercury.jpg", ORANGE),
		},
		Planet {
			col: 4,
			radius_m: 6_051e3,
			texture: tex("venus.jpg", ORANGE),
		},
		Planet {
			col: 5,
			radius_m: 6371e3,
			texture: tex("earth.jpg", BLUE),
		},
		Planet {
			col: 6,
			radius_m: 1738e3,
			texture: tex("moon.jpg", GRAY),
		},
		Planet {
			col: 7,
			radius_m: 3_396e3,
			texture: tex("mars.jpg", RED),
		},
		Planet {
			col: 8,
			radius_m: 69_911e3,
			texture: tex("jupiter.jpg", RED),
		},
	]
}

// A planet's properites
struct Planet {
	col: usize,                // body's index in the CSV input.
	radius_m: f64,             // radius in metres.
	texture: Arc<dyn Texture>, // texture or solid color fallback
}

// Try loading an image texture (e.g.: "mytex.jpg"),
// fall back to solid color on error.
fn tex(fname: &str, fallback: Color) -> Arc<dyn Texture> {
	match load(fname) {
		Ok(tex) => {
			println!("[ ] loaded {}", fname);
			Arc::new(tex)
		}
		Err(e) => {
			eprintln!("[!] {}", e.to_string());
			Arc::new(fallback)
		}
	}
}

// Parse body positions from CSV file.
// 3 columns per body with x, z , y coordinates, respectively.
// First line is ignored.
pub fn parse_positions_csv(fname: &str) -> Result<Vec<Vec<Point>>> {
	let fields = parse_fields(fname)?;
	let mut result = Vec::new();
	for fields in fields {
		result.push(vectorize(fields)?)
	}
	Ok(result)
}

// Parse a CSV file into a 2D matrix.
// First line is ignored.
fn parse_fields(fname: &str) -> Result<Vec<Vec<f64>>> {
	let mut result = Vec::new();

	let f = match File::open(fname) {
		Ok(f) => {
			println!("[ ] loaded {}", fname);
			f
		}
		Err(e) => return error(format!("loading {}: {}", fname, e.to_string())),
	};
	let reader = BufReader::new(f);
	for line in reader.lines().skip(1) {
		let line = line?;
		let fields = line.split(",");
		let mut values = Vec::new();
		for field in fields {
			if field == "" {
				break; // HACK for trailing ",".
			}
			let v: f64 = parse_f64(field.trim())?;
			values.push(v);
		}
		result.push(values);
	}

	Ok(result)
}

// Convert a row of numbers to a row of 3D points.
//     columns 0, 1, 2 make point 0
//     columns 3, 4, 5 make point 1
//     ...
fn vectorize(fields: Vec<f64>) -> Result<Vec<Point>> {
	let mut result = Vec::with_capacity(fields.len());

	if fields.len() % 3 != 0 {
		return error(format!("fields not a multiple of 3"));
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

// parse f64 with clear error message.
fn parse_f64(s: &str) -> Result<f64> {
	match s.parse() {
		Ok(v) => Ok(v),
		Err(e) => error(format!("parse {}: {}", s, e.to_string())),
	}
}
