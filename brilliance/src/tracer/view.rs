use super::*;

#[derive(Clone)]
pub struct View {
	pub camera: Camera,
	pub width: u32,
	pub height: u32,
	//pub recursion: i32,
	//pub num_pass: i32,
	//DebugNormals:     int
	//DebugIsometricFOV: float64
	//DebugIsometricDir: int
	//PostProcess: post.Params
}

impl View {
	pub fn dimensions(&self) -> (u32, u32) {
		(self.width, self.height)
	}
}
