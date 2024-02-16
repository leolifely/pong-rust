mod bat;
use bat::{Bat, Direction};
extern crate sdl2;
use sdl2::{pixels::Color, event::Event, keyboard::Keycode};
const SCREEN_SIZE: [i32; 2] = [1280, 720];

fn main() {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem.window("Pong", SCREEN_SIZE[0] as u32, SCREEN_SIZE[1] as u32)
		.position_centered()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().build().unwrap();
	canvas.set_draw_color(Color::RGB(0, 0, 0));
	canvas.clear();
	canvas.present();

	let mut event_pump = sdl_context.event_pump().unwrap();
	let mut bat_1 = Bat::new([50, 25], Color::RGB(0, 255, 0));
	let mut bat_2 = Bat::new([SCREEN_SIZE[0] - 75, 25], Color::RGB(255, 255, 255));

	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} => {
					break 'running
				},
				Event::KeyDown {keycode: Some(Keycode::Up), ..} => {
					bat_1.change_direction(Direction::Up);
				},
				Event::KeyDown {keycode: Some(Keycode::Down), ..} => {
					bat_1.change_direction(Direction::Down);
				},
				Event::KeyUp {keycode: Some(Keycode::Up), ..} | Event::KeyUp {keycode: Some(Keycode::Down), ..} => {
					bat_1.change_direction(Direction::Stationary);
				},
				Event::KeyDown {keycode: Some(Keycode::W), ..} => {
					println!("W pressed");
					bat_2.change_direction(Direction::Up);
				},
				Event::KeyDown {keycode: Some(Keycode::S), ..} => {
					println!("S pressed");
					bat_2.change_direction(Direction::Down);
				},
				Event::KeyUp {keycode: Some(Keycode::W), ..} | Event::KeyUp {keycode: Some(Keycode::S), ..} => {
					bat_2.change_direction(Direction::Stationary);
				},
				_ => {},
			}
		}
		bat_1.move_bat();
		bat_2.move_bat();

		bat_1.draw(&mut canvas);
		bat_2.draw(&mut canvas);
		canvas.present();
		canvas.set_draw_color(Color::RGB(0, 0, 0));
		canvas.clear();
	}
}