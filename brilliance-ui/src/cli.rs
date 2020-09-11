use super::*;

use std::env;
use std::ffi::OsString;
use std::io;
use std::io::Write;

pub fn spec_from_cli() -> Result<(Scene, View)> {
	let args: Vec<OsString> = env::args_os().skip(1).collect();
	if args.len() == 0 {
		return error("need 1 argument".into());
	}
	if args.len() != 1 {
		exit(&format!("need 1 argument, got {}", args.len()))
	}

	let fname = args[0].to_string_lossy();

	let obj = match parse_file(&fname) {
		Err(e) => exit(&format!("parsing {}: {}", &fname, e)),
		Ok(obj) => obj,
	};

	let s = Scene {
		max_iter: 1,
		max_recursion_depth: 3,
		ambient: Color::new(0.1, 0.1, 0.1),
		background: DynMaterial::new(Flat::new(Color::new(0.1, 0.1, 0.1))),
		objects: vec![DynObj::new(obj)],
		lights: vec![
			DynLight::new(PointLight::new(Point(1.0, 1.5, 1.5), Color::WHITE.ev(5.0))),
			//DynLight::new(PointLight::new(Point(-1.0, 1.0, 1.0), Color::WHITE.ev(3.0))),
		],
	};

	let c = Camera::pinhole(60.0 * DEG)
		.at(Point(0.0, 1.0, 2.0))
		.rot(Transform::yaw_pitch(0.0, -40.0 * DEG));

	let v = View {
		camera: c,
		width: 960,
		height: 540,
	};

	Ok((s, v))
}

//fn floor() -> DynObj {
//	parametric(matte(WHITE), (4, 4), |u, v| {
//		let x = (u - 0.5) * 4.0;
//		let y = (v - 0.5) * 4.0;
//		Point(x, 0.0, y)
//	})
//}

fn exit(msg: &str) -> ! {
	write!(io::stderr(), "{}", msg).unwrap();
	std::process::exit(1)
}
