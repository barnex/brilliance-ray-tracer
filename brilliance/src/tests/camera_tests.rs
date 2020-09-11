use super::*;

#[test]
fn look_at() {
	let diam = 0.10;
	test(
		"camera_look_at",
		&Scene {
			objects: vec![
				DynObj::new(Sphere::new(Point(0., 0., 0.), diam).paint(flat(WHITE))),
				DynObj::new(Sphere::new(Point(1., 0., 0.), diam).paint(flat(RED))),
				DynObj::new(Sphere::new(Point(0., 1., 0.), diam).paint(flat(GREEN))),
				DynObj::new(Sphere::new(Point(0., 0., 1.), diam).paint(flat(BLUE))),
			],
			..default_scene()
		},
		&View {
			camera: pinhole(120.0 * DEG)
				.at(Point(0.0, 0.5, -2.0))
				.look_at2(Point(1.0, 0.0, 0.0), Vector(1.0, 0.0, 0.0)),
			..default_view()
		},
	);
}

#[test]
fn look_dir() {
	let diam = 0.10;
	test(
		"camera_look_dir",
		&Scene {
			objects: vec![
				DynObj::new(Sphere::new(Point(1., 0., 0.), diam).paint(flat(RED))),
				DynObj::new(Sphere::new(Point(0., 1., 0.), diam).paint(flat(GREEN))),
				DynObj::new(Sphere::new(Point(0., 0., 1.), diam).paint(flat(BLUE))),
			],
			..default_scene()
		},
		&View {
			camera: pinhole(120.0 * DEG).look_dir(Point(0.8, 0.2, 1.0), Vector(0.0, 1.0, 0.0)),
			..default_view()
		},
	);
}

#[test]
// Test the pinhole camera by rendering 4 small spheres
// positioned at the origin, Ex, Ey and Ez.
// The resulting image should correspond to a right-handed coordinate system.
fn pinhole_handedness() {
	let diam = 0.10;
	test(
		"camera_pinhole_handedness",
		&Scene {
			objects: vec![
				DynObj::new(Sphere::new(Point(0., 0., 0.), diam).paint(flat(WHITE))),
				DynObj::new(Sphere::new(Point(1., 0., 0.), diam).paint(flat(RED))),
				DynObj::new(Sphere::new(Point(0., 1., 0.), diam).paint(flat(GREEN))),
				DynObj::new(Sphere::new(Point(0., 0., 1.), diam).paint(flat(BLUE))),
			],
			..default_scene()
		},
		&View {
			camera: pinhole(focallen_to_fov(1.0)).at(Point(0.0, 1.0, 5.0)),
			..default_view()
		},
	);
}

#[test]
// Test pinhole camera field of view.
// Place two spheres at unit distances from the orign.
// Set a non-trival focal length (2) and position (z=4) so that both spheres
// are exactly at the edges of the image:
//
//      1      1
//  x------o------x  object plane
//   \     |2    /
//    \ .5 | .5 /
//     x---o---x     image plane
//      \  |2 /
//       \ | /
//        cam
fn pinhole_fov() {
	let diam = 0.10;
	test(
		"camera_pinhole_fov",
		&Scene {
			objects: vec![
				DynObj::new(Sphere::new(Point(0., 0., 0.), diam).paint(flat(WHITE))),
				DynObj::new(Sphere::new(Point(1., 0., 0.), diam).paint(flat(RED))),
				DynObj::new(Sphere::new(Point(-1., 0., 0.), diam).paint(flat(GREEN))),
			],
			..default_scene()
		},
		&View {
			camera: pinhole(focallen_to_fov(2.0)).at(Point(0.0, 0.0, 4.0)),
			..default_view()
		},
	);
}

fn focallen_to_fov(l: f64) -> f64 {
	2.0 * f64::atan(0.5 / l)
}
