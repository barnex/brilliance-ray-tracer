use brilliance_ui::*;

use sdl2::event::Event;
use sdl2::mouse;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect;
use std::iter;

fn main() -> Result<()> {
	let (s, v) = spec_from_cli()?;
	let mut bakery = Bakery::new(s, v.dimensions());
	let (w, h) = v.dimensions();

	// initialize sdl window
	let context = sdl2::init()?;
	let window = context.video()?.window("brilliance-ui", w, h).resizable().position_centered().build()?;
	let mut canvas = window.into_canvas().accelerated().present_vsync().build()?;
	let mut tc = canvas.texture_creator();
	let mut event_pump = context.event_pump()?;

	'mainloop: loop {
		// TODO: only present of needed
		let (w, h) = canvas.output_size()?;
		let img = bakery.handle_draw((w, h));
		bakery.print_stats();
		draw(&mut canvas, &mut tc, &img);
		//println!("present");
		canvas.present();

		// wait for at least one event,
		// handle it and all pending events, if any.
		for event in iter::once(event_pump.wait_event()).chain(event_pump.poll_iter()) {
			match event {
				Event::Quit { .. } => break 'mainloop,
				event => handle_event(&mut bakery, event),
			}
		}
	}
	Ok(())
}

type Canvas = sdl2::render::Canvas<sdl2::video::Window>;
type TextureCreator = sdl2::render::TextureCreator<sdl2::video::WindowContext>;

fn draw(c: &mut Canvas, tc: &mut TextureCreator, img: &Image<Color>) {
	c.clear();

	// upload image to texture
	let tex = {
		let (w, h) = img.dimensions();
		let mut tex = tc.create_texture_streaming(PixelFormatEnum::BGRA32, w as u32, h as u32).unwrap();

		tex.update(None, &img.raw_bgra(), 4 * w as usize).unwrap();
		tex
	};

	// stretch to screen size while keeping aspect ratio.
	let screensz = c.output_size().unwrap();
	let mut imgsz = img.dimensions();
	while 2 * imgsz.0 <= screensz.0 && 2 * imgsz.1 <= screensz.1 {
		imgsz.0 *= 2;
		imgsz.1 *= 2;
	}

	// draw texture
	let dst = Some(rect::Rect::new(0, 0, imgsz.0, imgsz.1));
	c.copy(&tex, None, dst).unwrap()
}

fn handle_event(b: &mut Bakery, event: sdl2::event::Event) {
	match event {
		Event::MouseMotion { x, y, mousestate, .. } => b.mouse_motion((x, y), mousestate.left(), mousestate.right()),
		Event::MouseButtonDown { x, y, mouse_btn, .. } => {
			b.mouse_down((x, y), mouse_btn == mouse::MouseButton::Left, mouse_btn == mouse::MouseButton::Right)
		}
		Event::MouseButtonUp { x, y, mouse_btn, .. } => {
			b.mouse_up((x, y), mouse_btn == mouse::MouseButton::Left, mouse_btn == mouse::MouseButton::Right)
		}
		Event::MouseWheel { x, y, .. } => b.mouse_wheel((x, y)),
		Event::KeyDown { keycode, .. } => {
			if let Some(keycode) = keycode {
				b.key_down(keymap(keycode));
			}
		}
		Event::KeyUp { keycode, .. } => {
			if let Some(keycode) = keycode {
				b.key_up(keymap(keycode));
			}
		}
		_ => (),
	}
}

fn keymap(sdl_key: sdl2::keyboard::Keycode) -> Key {
	use sdl2::keyboard::Keycode;
	match sdl_key {
		Keycode::Left => Key::Left,
		Keycode::S => Key::Left,
		Keycode::Right => Key::Right,
		Keycode::F => Key::Right,
		Keycode::Up => Key::Forward,
		Keycode::E => Key::Forward,
		Keycode::Down => Key::Backward,
		Keycode::D => Key::Backward,
		Keycode::Space => Key::Up,
		Keycode::Z => Key::Down,
		Keycode::Equals => Key::ZoomIn,
		Keycode::Minus => Key::ZoomOut,
		Keycode::P => Key::Pause,
		_ => Key::None,
	}
}
