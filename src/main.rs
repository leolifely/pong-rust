mod bat;
mod ball;

use bat::{Bat, Direction};
use ball::{Ball, Collision};
extern crate sdl2;
use sdl2::{pixels::Color, event::Event, keyboard::Keycode};
use std::time::{Duration, SystemTime};

const SCREEN_SIZE: [i32; 2] = [1280, 720];

fn main() {
	// Set up the window and the canvas
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

	// Set up the event pump
	let mut event_pump = sdl_context.event_pump().unwrap();
	// Set up the game objects
	let mut bat_1 = Bat::new([50, 25], [25, 100], Color::RGB(0, 255, 0));
	let mut bat_2 = Bat::new([SCREEN_SIZE[0] - 75, 25], [25, 100], Color::RGB(255, 255, 255));
	let mut ball = Ball::new([SCREEN_SIZE[0] / 2, SCREEN_SIZE[1] / 2], [1, 1], Color::RGB(255, 255, 0));

	// Main game loop
	let mut now = SystemTime::now();
	'running: loop {
		// Handle events
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} => {
					break 'running
				},
				Event::KeyDown {keycode: Some(Keycode::Up), ..} => {
					bat_2.change_direction(Direction::Up);
				},
				Event::KeyDown {keycode: Some(Keycode::Down), ..} => {
					bat_2.change_direction(Direction::Down);
				},
				Event::KeyUp {keycode: Some(Keycode::Up), ..} | Event::KeyUp {keycode: Some(Keycode::Down), ..} => {
					bat_2.change_direction(Direction::Stationary);
				},
				Event::KeyDown {keycode: Some(Keycode::W), ..} => {
					bat_1.change_direction(Direction::Up);
				},
				Event::KeyDown {keycode: Some(Keycode::S), ..} => {
					bat_1.change_direction(Direction::Down);
				},
				Event::KeyUp {keycode: Some(Keycode::W), ..} | Event::KeyUp {keycode: Some(Keycode::S), ..} => {
					bat_1.change_direction(Direction::Stationary);
				},
				_ => {},
			}
		}
		// Move the game objects
		if now.elapsed().unwrap().as_millis() > 1 {
			bat_1.move_bat(SCREEN_SIZE);
			bat_2.move_bat(SCREEN_SIZE);
			ball.move_ball();
			println!("{:?}", ball.test_collision(&bat_1, &bat_2, SCREEN_SIZE));
			now = SystemTime::now();
		}
		// Draw the game objects
		bat_1.draw(&mut canvas);
		bat_2.draw(&mut canvas);
		ball.draw(&mut canvas);
		canvas.present();
		canvas.set_draw_color(Color::RGB(0, 0, 0));
		canvas.clear();
	}
}