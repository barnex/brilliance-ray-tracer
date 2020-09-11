use super::*;

#[derive(Clone)]
pub struct Camera {
	projection: Projection,
	pub position: Point,
	rotation: Matrix<f64>,
}

#[derive(Clone)]
enum Projection {
	Pinhole { focal_len: f64 },
}

use Projection::*;

impl Camera {
	pub fn pinhole(fov_radians: f64) -> Self {
		Self::new(Pinhole {
			focal_len: fov_to_focal_len(fov_radians),
		})
	}

	fn new(p: Projection) -> Self {
		Self {
			projection: p,
			position: Point::default(),
			rotation: Matrix::from([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, -1.0]]),
		}
	}

	pub fn at(self, position: Point) -> Self {
		Self { position, ..self }
	}

	/// Pitch+Yaw the camera to look a point `look_at`,
	/// while keeping the vertical direction close to Y.
	pub fn look_at(self, look_at: Point) -> Self {
		self.look_at2(look_at, Vector::EY)
	}

	/// Pitch+Yaw the camera to look a point `look_at`,
	/// Roll so that direction `up` is as close as possible to the up direction
	/// (typically Y).
	pub fn look_at2(self, look_at: Point, up: Vector) -> Self {
		let pos = self.position;
		self.look_dir(look_at - pos, up)
	}

	/// Like `look_at2`, but looking into a direction (i.e. independent of camera position)
	/// rather than to a point (i.e. relative to the camera).
	pub fn look_dir(self, look_dir: Vector, up: Vector) -> Self {
		if look_dir.cross(up).len() < 1e-6 {
			panic!(
				"illegal argument: Camera: look_dir: must not be parallel with up. got: {}, {}",
				look_dir, up
			);
		}
		let z = look_dir.normalized();
		let x = z.cross(up).normalized();
		let y = x.cross(z);
		let rotation = Matrix::new(x, y, z);
		Self { rotation, ..self }
	}

	pub fn rot(self, rotation: Matrix<f64>) -> Self {
		Self { rotation, ..self }
	}

	pub fn yaw_pitch(self, yaw_radians: f64, pitch_radians: f64) -> Self {
		self.rot(Transform::yaw_pitch(yaw_radians, pitch_radians))
	}

	pub fn ray_from(&self, rng: &mut Rng, uv: (f64, f64)) -> Ray {
		let orig = self.projection.ray_from(rng, uv);
		Ray::new(orig.start + self.position, self.rotation * orig.dir)
	}
}

impl Projection {
	fn ray_from(&self, _rng: &mut Rng, (u, v): (f64, f64)) -> Ray {
		debug_assert!(valid_uv(u));
		debug_assert!(valid_uv(v));

		let x = u - 0.5;
		let y = v - 0.5;

		match *self {
			Pinhole { focal_len } => Ray::new(Point(0.0, 0.0, 0.0), Vector(x, y, focal_len).normalized()),
		}
	}
}

// fovToFocalLen converts a Field Of View (in radians) to focal length
// corresponding to a sensor of size 1.
//
// 	FOV = 2*atan(f/2)
fn fov_to_focal_len(fov_radians: f64) -> f64 {
	assert!(fov_radians > 0.0);
	assert!(fov_radians < PI);
	return 0.5 / f64::tan(fov_radians / 2.0);
}

//pub struct Isometric {
//	size: f64,
//	offset: f64,
//}
//
//impl Isometric {
//	pub fn new(size: f64) -> Isometric {
//		Isometric {
//			size: size,
//			offset: 4096.,
//		}
//	}
//}
//
//impl Camera for Isometric {
//	fn ray_from(&self, u: f64, v: f64) -> Ray {
//		let s = self.size;
//		Ray::new(
//			Point((u - 0.5) * s, (v - 0.5) * s, self.offset),
//			Vector(0., 0., -1.),
//		)
//	}
//}

// // Translate returns an instance of this camera whose position has been translated.
// // The original is not affected.
// // The delta is in absolute coordinates, unaffected by the camera's view direction.
// func (c *WithTransform) Translate(delta Vec) *WithTransform {
// 	return translate(c, delta)
// }

/// indexToCam maps a pixel index {ix, iy} inside an image with given width and height
/// onto a u,v coordinate strictly inside the interval [0,1].
/// If the image's aspect ratio width:height is not square,
/// then either u or v will not span the entire [0,1] interval.
///
/// Half-pixel offsets are applied so that the borders in u,v correspond
/// exactly to pixel borders (not centers). This transformation is sketched below:
///
///
/// Note that the v axis points up, while the y axis points down.
pub fn index_to_cam((w, h): (u32, u32), (ix, iy): (f64, f64)) -> (f64, f64) {
	//              +----------------+ (u,v=1,1)
	//              |                |
	// (x,y=-.5,-.5)+----------------+
	//              |                |
	//              |                |
	//              +----------------+ (x,y=w-.5,h-.5)
	//              |                |
	//     (u,v=0,0)+----------------+
	let w = w as f64;
	let h = h as f64;

	debug_assert!(!(ix < -0.5 || iy < -0.5 || ix > w - 0.5 || iy > h - 0.5));

	let u = linterp(-0.5, 0.0, w - 0.5, 1.0, ix);
	let v = linterp(-0.5, 0.5 + 0.5 * (h / w), h - 0.5, 0.5 - 0.5 * (h / w), iy);
	(u, v)
}

// linear interpolation
// 	x1 -> y1
// 	x2 -> y2
// 	x  -> y
fn linterp(x1: f64, y1: f64, x2: f64, y2: f64, x: f64) -> f64 {
	y1 + (y2 - y1) * (x - x1) / (x2 - x1)
}

fn valid_uv(u: f64) -> bool {
	(u >= 0.) && (u <= 1.)
}
