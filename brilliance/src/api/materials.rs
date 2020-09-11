use crate::*;

pub const WHITE: Color = Color::WHITE;
pub const BLACK: Color = Color::BLACK;
pub const RED: Color = Color::RED;
pub const GREEN: Color = Color::GREEN;
pub const BLUE: Color = Color::BLUE;
pub const YELLOW: Color = Color::YELLOW;
pub const ORANGE: Color = Color::ORANGE;
pub const GRAY: Color = Color::GRAY;

pub fn matte<T: Texture + 'static>(tex: T) -> DynMaterial {
	DynMaterial::new(Matte::new(tex))
}

pub fn flat<T: Texture + 'static>(tex: T) -> DynMaterial {
	DynMaterial::new(Flat::new(tex))
}
