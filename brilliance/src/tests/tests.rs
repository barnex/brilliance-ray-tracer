use super::*;

#[test]
fn render_sinc() {
	// TODO: test camera yaw, pitch separately first.
	//test(
	//	"sinc",
	//	&Scene {
	//		objects: vec![floor(4.0, 4.0), sinc(64)],
	//		lights: vec![
	//			point_light(Point(1.0, 1.0, 1.0), WHITE.ev(5.0)),
	//			point_light(Point(-1.0, 2.0, 1.0), WHITE.ev(5.0)),
	//		],
	//		..default_scene()
	//	},
	//	&View {
	//		camera: pinhole(60.0 * DEG).at(Point(0.0, 1.0, 2.0)).yaw_pitch(180.0 * DEG, 40.0 * DEG),
	//		..default_view()
	//	},
	//);
}

#[test]
fn render_f32bleed() {
	// When ray-face intersection is done in f32 precision,
	// and the ray starts far away from the face (>8.0 seems enough)
	// insuffcient precision on the intersection point may cause
	// shadow bleeding.
	test(
		"f32bleed",
		&Scene {
			objects: vec![floor(4.0, 4.0), sinc(64)],
			lights: vec![
				point_light(Point(1.0, 1.0, 1.0), WHITE.ev(5.0)),
				point_light(Point(-1.0, 2.0, 1.0), WHITE.ev(5.0)),
			],
			..default_scene()
		},
		&View {
			camera: pinhole(5.0 * DEG).at(Point(0.0, 0.0, 30.0)), // far away, tele lens
			..default_view()
		},
	);
}

fn floor(width: f64, depth: f64) -> DynObj {
	parametric(matte(WHITE), (2, 2), |u, v| {
		let x = (u - 0.5) * width;
		let y = (v - 0.5) * depth;
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
