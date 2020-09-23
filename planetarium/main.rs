use brilliance::*;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;
extern crate structopt;
use structopt::StructOpt;

// Astronomic unit. All positions scaled by this to keep numbers reasonable.
static AU: f64 = 150e9;

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

	/// Start at this frame.
	#[structopt(short, long, default_value = "0")]
	start_frame: usize,

	/// Number of samples per pixel, improves quality.
	#[structopt(short, long, default_value = "1")]
	samples: u32,

	/// Activate top view for debugging.
	#[structopt(short, long)]
	topview: bool,

	/// Camera field of view (degrees).
	#[structopt(long, default_value = "60")]
	fov: f64,

	/// Camera follows this target object.
	#[structopt(long, default_value = "0")]
	follow: usize,

	/// Camera follows target (--follow) by this distance (km).
	#[structopt(long, default_value = "10000")]
	follow_km: f64,

	/// Optional, camera looks at this object.
	#[structopt(long)]
	look_at: Option<usize>,

	/// Disable shadows.
	#[structopt(long)]
	disable_shadows: bool,

	/// Amount of ambient background light (0..1).
	#[structopt(long, default_value = "0.01")]
	ambient_light: f32,

	/// Depth of recursive ray tracing.
	#[structopt(long, default_value = "0")]
	recursion: u32,

	/// Scale body sizes by this factor.
	#[structopt(long, default_value = "1")]
	scale_bodies: f64,

	/// Controls the sun intensity, exponential scale.
	#[structopt(long, default_value = "6")]
	sun_ev: f64,
}

fn main() -> Result<()> {
	let args = Cli::from_args();
	println!("[I] ouput directory: {}", args.output);
	println!("[I] ouput size: {} x {}", args.width, args.height);
	if let Err(e) = fs::create_dir(&args.output) {
		eprintln!("[?] create directory {}: {}", &args.output, e.to_string())
	}

	// read planet positions from file
	let (body_propts, all_positions) = parse_csv(&args.input)?;
	for body in &body_propts {
		println!("[I] body {}: {}\t r: {} km", body.col, body.name, body.radius_m / 1e3);
	}
	println!(
		"[I] camera follows body {} ({}) from {} km",
		body_propts[args.follow].col, &body_propts[args.follow].name, args.follow_km
	);

	let total_frames = all_positions.len() / args.every;
	println!("[I] input frames: {}", all_positions.len());
	if args.every != 1 {
		println!("[I] rendering every {}th frame, total frames: {}", args.every, total_frames);
	}

	// prepare planet properties and load textures
	//let planet_propts = planet_propts();

	// Optionally scale planet sizes to make better visible.
	let scale_planets = args.scale_bodies;

	// Milky way background
	let backdrop: Arc<dyn Texture> = if args.topview { Arc::new(BLACK) } else { tex(&args.background, BLACK) };

	// Animation: one render per line in the input.
	for (i, positions) in all_positions.iter().enumerate().skip(args.start_frame).step_by(args.every) {
		// Celestial body models
		let mut objects = Vec::new();

		// Add the sun.
		//let diam_sun = 2.0 * 695_700e3 * scale_sun / AU;
		//objects.push(DynObj::new(Sphere::new(Point(0., 0., 0.), diam_sun).paint(flat(YELLOW))));

		// Add the planets
		for p in body_propts.iter().skip(1) {
			add_planet(&mut objects, p, &positions, &args);
		}

		//let sun = DynLight::new(PointLight::new(Point(0.0, 0.1, 0.0), Color::new(1.0, 0.95, 0.70).ev(5.5)));
		let sun_pos = Point(0., 0., 0.);
		let sun_color = Color::new(1.0, 0.95, 0.70);
		let sun = sphere_light(sun_pos, 2.0 * body_propts[0].radius_m / AU, sun_color.ev(args.sun_ev));

		let s = Scene {
			max_recursion_depth: args.recursion,
			max_iter: args.samples,
			ambient: WHITE * args.ambient_light,
			background: flat(UVMapped::sphere(backdrop.clone())),
			//objects: vec![DynObj::new(QTree::new(objects))], // TODO: occlusion bug
			objects,
			lights: vec![sun], // TODO
		};

		// Set the camera
		let c = if args.topview {
			Camera::pinhole(30.0 * DEG)
				.at(Point(0.0, 25.0, 0.0))
				.look_at2(Point(0., 0., 0.), Vector(0.0, 0.0, 1.0))
		} else {
			let follow_pos = all_positions[i][args.follow] / AU;
			let follow_delta = if i == 0 {
				let follow_next = all_positions[i + 1][args.follow] / AU;
				follow_next - follow_pos
			} else {
				let follow_prev = all_positions[i - 1][args.follow] / AU;
				follow_pos - follow_prev
			};
			let follow_dir = follow_delta.normalized();
			let cam_pos = follow_pos - follow_dir * args.follow_km * 1e3 / AU;

			if let Some(body) = args.look_at {
				let target = all_positions[i][body] / AU;
				Camera::pinhole(args.fov * DEG).at(cam_pos).look_at2(target, Vector::EY)
			} else {
				Camera::pinhole(args.fov * DEG).at(cam_pos).look_dir(follow_dir, Vector::EY)
			}
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
		let img = add_gaussian_bloom(&img, 0.01, 1.5);
		let img = add_gaussian_bloom(&img, 0.004, 12.0);
		//let img = add_bleed(&img, 1.0);
		let fname = format!("{}{}{:05}.jpg", &args.output, std::path::MAIN_SEPARATOR, i);
		match save_jpg(&img, &fname, quality) {
			Ok(()) => Ok(()),
			Err(e) => error(format!("save {}: {}", &fname, e.to_string())),
		}?;
		println!("[V] rendered {}", &fname);
	}

	Ok(())
}

fn add_planet(objects: &mut Vec<DynObj>, p: &Planet, positions: &Vec<Point>, args: &Cli) {
	let diam = if args.topview { 0.1 } else { 2.0 * p.radius_m * args.scale_bodies / AU };
	let pos = positions[p.col] / AU;

	//if p.name == "Meteorite" {
	//	return objects.push(meteorite(&p, pos, &args));
	//}

	//dbg!(p.col, pos, diam);
	let mat = if args.disable_shadows {
		flat(p.texture.clone())
	} else {
		matte(p.texture.clone())
	};
	objects.push(DynObj::new(Sphere::new(pos, diam).paint(mat)));
	//if p.name == "Earth" {
	//	// atmosphere
	//	objects.push(DynObj::new(
	//		Sphere::new(pos, diam + 600e3 / AU).paint(Translucent::new(WHITE, Color::new(0.5, 0.5, 1.0).ev(-6.0))),
	//	));
	//}
}

fn meteorite(p: &Planet, pos: Point, args: &Cli) -> DynObj {
	//DynObj::new(Sphere::new(pos, diam).paint(mat)));
	//let obj = parse_file("meteorite.obj").expect("load meteorite.obj");
	//DynObj::new(TransformedObj::new(obj, 1.0, pos))
	DynObj::new(TransformedObj::new(
		//Sphere::new(Point(0.0, 0.0, 0.0), 1.0).paint(matte(WHITE)),
		parse_file("meteorite.obj").expect("load meteorite.obj"),
		p.radius_m / AU,
		pos,
	))
}

// A planet's properites
struct Planet {
	col: usize, // body's index in the CSV input.
	name: String,
	radius_m: f64,             // radius in metres.
	texture: Arc<dyn Texture>, // texture or solid color fallback
}

// Try loading an image texture (e.g.: "mytex.jpg"),
// fall back to solid color on error.
fn tex(fname: &str, fallback: Color) -> Arc<dyn Texture> {
	match load(fname) {
		Ok(tex) => {
			println!("[V] loaded {}", fname);
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
// First has `planet_name:radius`.
fn parse_csv(fname: &str) -> Result<(Vec<Planet>, Vec<Vec<Point>>)> {
	let (header, fields) = parse_fields(fname)?;
	let mut positions = Vec::new();
	for fields in fields {
		positions.push(vectorize(fields)?)
	}
	Ok((header, positions))
}

// Parse a CSV file into a 2D matrix and header.
fn parse_fields(fname: &str) -> Result<(Vec<Planet>, Vec<Vec<f64>>)> {
	let f = match File::open(fname) {
		Ok(f) => {
			println!("[V] loaded {}", fname);
			f
		}
		Err(e) => return error(format!("loading {}: {}", fname, e.to_string())),
	};
	let reader = BufReader::new(f);
	let mut lines = reader.lines();

	let header = lines.next().unwrap()?; // TODO: clear error on empty file
	let header = parse_header(&header)?;

	let mut positions = Vec::new();
	for line in lines {
		let line = line?;
		let fields = line.trim_end_matches(",").split(",");
		let mut values = Vec::new();
		for field in fields {
			//if field == "" {
			//	break; // HACK for trailing ",".
			//}
			let v: f64 = parse_f64(field.trim())?;
			values.push(v);
		}
		positions.push(values);
	}

	Ok((header, positions))
}

fn parse_header(line: &str) -> Result<Vec<Planet>> {
	let mut planets = Vec::new();
	for (i, field) in line.trim_end_matches(",").split(",").enumerate() {
		let field = field.trim();
		//dbg!(i, field);
		if field == "" {
			break; // HACK
		}
		let fields: Vec<&str> = field.split(":").collect();
		let name = fields[0].trim();
		let radius_m: f64 = fields[1].trim().parse()?;
		let texture_file = &(name.to_ascii_lowercase() + ".jpg");
		planets.push(Planet {
			col: i,
			texture: tex(texture_file, GRAY),
			radius_m,
			name: name.into(),
		});
	}
	Ok(planets)
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
