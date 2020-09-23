use super::*;

#[test]
fn with_object() {
	let diam = 0.50;
	let off = Point(0.0, -0.5, -2.0);
	test(
		"light_with_object",
		&Scene {
			objects: vec![
				DynObj::new(Sphere::new(Point(0., 0., 0.) + off, diam).paint(matte(WHITE))),
				DynObj::new(Sphere::new(Point(1., 0., 0.) + off, diam).paint(matte(RED))),
				DynObj::new(Sphere::new(Point(0., 1., 0.) + off, diam).paint(matte(GREEN))),
				DynObj::new(Sphere::new(Point(0., 0., 1.) + off, diam).paint(matte(BLUE))),
			],
			lights: vec![DynLight::new(WithObject::new(
				PointLight::new(Point(0.5, 0.5, 0.5) + off, WHITE.ev(4.0)),
				Sphere::new(Point(0.5, 0.5, 0.5) + off, diam).paint(flat(WHITE.ev(4.0))),
			))],
			..default_scene()
		},
		&View {
			camera: pinhole(120.0 * DEG),
			..default_view()
		},
	);
}

#[test]
fn point() {
	let diam = 0.50;
	let off = Point(0.0, -0.5, -2.0);
	test(
		"light_point",
		&Scene {
			objects: vec![
				DynObj::new(Sphere::new(Point(0., 0., 0.) + off, diam).paint(matte(WHITE))),
				DynObj::new(Sphere::new(Point(1., 0., 0.) + off, diam).paint(matte(RED))),
				DynObj::new(Sphere::new(Point(0., 1., 0.) + off, diam).paint(matte(GREEN))),
				DynObj::new(Sphere::new(Point(0., 0., 1.) + off, diam).paint(matte(BLUE))),
			],
			lights: vec![DynLight::new(PointLight::new(Point(0.5, 0.5, 0.5) + off, WHITE.ev(4.0)))],
			..default_scene()
		},
		&View {
			camera: pinhole(120.0 * DEG),
			..default_view()
		},
	);
}
