use super::internal;
use super::*;

/// ShadingNormal is a non-physical material that reveals the component of the shading normal vector
/// that points towards the camera.
/// The normal pointing perpendicular to the direction of view is rendered as black.
/// Away from the viewing direction is rendered as red to draw attention to visible backfaces.
pub struct ShadingNormal();

impl Material for ShadingNormal {
	fn shade(&self, _: &Scene, r: &Ray, h: &HitCoords, _: &mut Rng, _depth: u32) -> Color {
		internal::shade_normal(r, &h.shading_normal())
	}
}
