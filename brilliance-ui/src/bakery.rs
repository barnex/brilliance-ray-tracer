use super::*;
use std::time::{Duration, Instant};
extern crate num_cpus;

pub struct Bakery {
	scene: Scene,
	cam_fov: f64,
	cam_pos: Point,
	cam_dir: (f64, f64),
	dimensions: (u32, u32),
	prev_mouse: Option<(i32, i32)>,
	last_wall: Duration,
}

impl Bakery {
	pub fn new(scene: Scene, dim: (u32, u32)) -> Self {
		Self {
			scene,
			cam_fov: 60.0 * DEG,
			cam_pos: Point(0.0, 0.5, 5.0),
			dimensions: dim,
			last_wall: Duration::from_secs(0),
			cam_dir: (0.0, 0.0),
			prev_mouse: None,
		}
	}

	// TODO: size of window, not spec, but respect aspect ratio
	pub fn handle_draw(&mut self, (_w, _h): (u32, u32)) -> Image<Color> {
		let start = Instant::now();

		let img = render(&self.scene, &self.view(), num_threads());

		self.last_wall = start.elapsed();
		img
	}

	fn view(&self) -> View {
		View {
			camera: self.camera(),
			width: self.dimensions.0,
			height: self.dimensions.1,
		}
	}

	fn camera(&self) -> Camera {
		Camera::pinhole(self.cam_fov).rot(self.cam_rotation()).at(self.cam_pos)
	}

	fn cam_rotation(&self) -> Matrix<f64> {
		Transform::yaw_pitch(self.cam_dir.0, self.cam_dir.1)
	}

	pub fn mouse_motion(&mut self, (x, y): (i32, i32), left: bool, _right: bool) {
		if left {
			let sens = 0.001;
			if let Some(down) = self.prev_mouse {
				let (dx, dy) = (x - down.0, y - down.1);
				self.cam_dir.0 += (dx as f64) * sens;
				if self.cam_dir.0 < -PI {
					self.cam_dir.0 += 2. * PI;
				}
				if self.cam_dir.0 > PI {
					self.cam_dir.0 -= 2. * PI;
				}
				self.cam_dir.1 += (dy as f64) * sens;
				self.cam_dir.1 = clamp(self.cam_dir.1, -PI / 2., PI / 2.);
				self.prev_mouse = Some((x, y));
			}
		}
	}

	pub fn mouse_up(&mut self, (_x, _y): (i32, i32), _left: bool, _right: bool) {
		self.prev_mouse = None
	}

	pub fn mouse_down(&mut self, pos: (i32, i32), _left: bool, _right: bool) {
		self.prev_mouse = Some(pos)
	}

	pub fn mouse_wheel(&mut self, (x, y): (i32, i32)) {
		let sens = 0.1;
		self.move_cam(Vector(-x as f64, 0.0, -y as f64) * sens)
	}

	pub fn key_down(&mut self, k: Key) {
		let move_dir = match k {
			Key::Left => -Vector::EX,
			Key::Right => Vector::EX,
			Key::Forward => -Vector::EZ,
			Key::Backward => Vector::EZ,
			Key::Up => Vector::EY,
			Key::Down => -Vector::EY,
			_ => Vector::default(),
		};
		let sens = 0.05;
		self.move_cam(move_dir * sens);

		let zoom = match k {
			Key::ZoomOut => 5.0 * DEG,
			Key::ZoomIn => -5.0 * DEG,
			_ => 0.0,
		};
		self.cam_fov = clamp(self.cam_fov + zoom, 5.0 * DEG, 175.0 * DEG);
	}

	/// Move camera relative to its own frame (look dir).
	/// E.g.: moving along (0, 0, 1) moves along the camea's view direction
	/// (not absolute Z axis, unless the camera has not been rotated).
	fn move_cam(&mut self, dir: Vector) {
		self.cam_pos += self.cam_rotation() * dir
	}

	pub fn key_up(&mut self, _k: Key) {}

	pub fn print_stats(&self) {
		// clear terminal
		use std::io::Write;
		std::io::stdout().write_all(b"\x1B[2J\x1B[H").unwrap();

		println!(
			"cam: ({:+3.2} {:+3.2} {:+3.2}), dir: ({:+.1}, {:+.1}) deg, fov: {:3.1} deg",
			self.cam_pos[0],
			self.cam_pos[1],
			self.cam_pos[2],
			self.cam_dir.0 / DEG,
			self.cam_dir.1 / DEG,
			self.cam_fov / DEG
		);

		println!("render: {:.1} ms", 1000.0 * self.last_wall.as_secs_f64(),);
	}
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
	if x < min {
		return min;
	}
	if x > max {
		return max;
	}
	x
}

fn num_threads() -> u32 {
	num_cpus::get_physical() as u32
}
