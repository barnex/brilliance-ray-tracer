/// Logical key codes, after being mapped from physical keys (by fn keymap).
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Key {
	None,
	Left,
	Right,
	Forward,
	Backward,
	Up,
	Down,
	ZoomIn,
	ZoomOut,
	Pause,
}
