use super::*;

pub fn test(name: &str, scene: &Scene, view: &View) {
	let got = render(scene, view, num_cpu());
	save(&got, &got_file(name)).expect("save image");
	let want = load(&want_file(name)).expect("load golden image");

	assert_eq!(got.dimensions(), want.dimensions());
	let (w, h) = got.dimensions();

	let mut diff: u64 = 0;
	for y in 0..h {
		for x in 0..w {
			diff += abs_diff(got.at((x, y)).srgb(), want.at((x, y)));
		}
	}

	assert_eq!(diff, 0);
}

pub fn default_scene() -> Scene {
	Scene {
		max_iter: 1,
		max_recursion_depth: 0,
		ambient: Color::BLACK,
		background: DynMaterial::new(Flat::new(Color::BLACK)),
		objects: vec![],
		lights: vec![point_light(Point(0.0, 20.0, 0.0), WHITE.ev(10.0))],
	}
}

pub fn default_view() -> View {
	View {
		camera: Camera::pinhole(60.0 * DEG)
			.at(Point(0.0, 1.0, 4.0))
			.rot(Transform::yaw_pitch(0.0, 0.0 * DEG)),
		width: 1920 / 4,
		height: 1080 / 4,
	}
}

fn abs_diff(a: RGB, b: RGB) -> u64 {
	(abs(a[0] as i32 - b[0] as i32) + abs(a[1] as i32 - b[1] as i32) + abs(a[2] as i32 - b[2] as i32)) as u64
}

fn abs(x: i32) -> i32 {
	if x < 0 {
		-x
	} else {
		x
	}
}

pub fn num_cpu() -> u32 {
	8 // TODO
}

fn got_file(base: &str) -> String {
	"testdata/".to_string() + base + "_got.png"
}

fn want_file(base: &str) -> String {
	"testdata/".to_string() + base + "_golden.png"
}
