use super::*;

pub fn test_spec(n: u32) -> (Scene, View) {
	let s = Scene {
		max_recursion_depth: 3,
		max_iter: 7,
		ambient: Color::new(0.1, 0.1, 0.1),
		background: DynMaterial::new(Flat::new(Color::new(0.1, 0.1, 0.1))),
		objects: vec![sinc(n), floor()],
		lights: vec![
			DynLight::new(PointLight::new(Point(1.0, 1.0, 1.0), Color::WHITE.ev(5.0))),
			DynLight::new(PointLight::new(Point(-1.0, 2.0, 1.0), Color::WHITE.ev(5.0))),
		],
	};

	let c = Camera::pinhole(60.0 * DEG)
		.at(Point(0.0, 1.0, 2.0))
		.rot(Transform::yaw_pitch(0.0, -40.0 * DEG));

	let v = View {
		camera: c,
		width: 1920 / 2,
		height: 1080 / 2,
	};

	(s, v)
}

//fn origin() -> Point {
//    Point(0.0, 0.0, 0.0)
//}

fn floor() -> DynObj {
	parametric(matte(WHITE), (4, 4), |u, v| {
		let x = (u - 0.5) * 4.0;
		let y = (v - 0.5) * 4.0;
		Point(x, -0.7, y)
	})
}

fn sinc(n: u32) -> DynObj {
	parametric(matte(WHITE), (n, n), |u, v| {
		let x = u - 0.5;
		let y = v - 0.5;
		let r = f64::sqrt(x * x + y * y) * 15.0;
		let h = 0.5;
		let z = h * if r == 0.0 { 1.0 } else { f64::sin(r) / r };
		Point(-x, z, y) - Vector(0., 0.5, 0.)
	})
}
