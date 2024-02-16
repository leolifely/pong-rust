mod bat;
use bat::{Bat, Direction};
extern crate sdl2;
use sdl2::{pixels::Color, event::Event, keyboard::Keycode, rect};
const SCREEN_SIZE: [i32; 2] = [1280, 720];

fn main() {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem.window("Snake Game", SCREEN_SIZE[0] as u32, SCREEN_SIZE[1] as u32)
		.position_centered()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().build().unwrap();
	canvas.set_draw_color(Color::RGB(0, 0, 0));
	canvas.clear();
	canvas.present();

	let mut event_pump = sdl_context.event_pump().unwrap();
	let mut bat = Bat::new([50, 25], Color::RGB(0, 255, 0));

	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} => {
					break 'running
				},
				Event::KeyDown {keycode: Some(Keycode::Up), ..} => {
					bat.change_direction(Direction::Up);
				},
				Event::KeyDown {keycode: Some(Keycode::Down), ..} => {
					bat.change_direction(Direction::Down);
				},
				_ => {
					bat.change_direction(Direction::Stationary);
				}

			}
		}
		bat.move_bat();
		canvas.set_draw_color(bat.get_colour());
		let bat_rect = rect::Rect::new(bat.get_position()[0], bat.get_position()[1], 25, 100);
		canvas.fill_rect(bat_rect).unwrap();
		canvas.present();
	}
}