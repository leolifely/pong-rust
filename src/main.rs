mod bat;
mod ball;

use std::path::Path;
use bat::{Bat, Direction};
use ball::{Ball, Collision};
extern crate sdl2;
use sdl2::{pixels::Color, event::Event, keyboard::Keycode, ttf};
use std::time::SystemTime;

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
			ball.update_color(SCREEN_SIZE);
			match ball.test_collision(&bat_1, &bat_2, SCREEN_SIZE) {
				Collision::LeftSide => {bat_2.increase_score();println!("Score: {} - {}", bat_1.get_score(), bat_2.get_score());},
				Collision::RightSide => {bat_1.increase_score();println!("Score: {} - {}", bat_1.get_score(), bat_2.get_score());},
				_ => (),
			}
			now = SystemTime::now();
		}
		// Draw the game objects
		bat_1.update_color(SCREEN_SIZE, ball.get_position());;
		bat_1.draw(&mut canvas);
		bat_2.update_color(SCREEN_SIZE, ball.get_position());
		bat_2.draw(&mut canvas);
		ball.draw(&mut canvas);
		draw_scores(bat_1.get_score(), bat_2.get_score(), &mut canvas, SCREEN_SIZE);
		canvas.present();
		canvas.set_draw_color(Color::RGB(0, 0, 0));
		canvas.clear();
	}
}

fn draw_scores(score_1: i32, score_2: i32, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, screen_size: [i32; 2]) {
	let font_path: &Path = Path::new("font/7_seg.ttf");
	let texture_creator = canvas.texture_creator();
	let ttf_context = ttf::init().unwrap();
	let font = ttf_context.load_font(font_path, 128).unwrap();

	let surface_1 = font.render(&score_1.to_string()).blended(Color::RGBA(0, 255, 0, 255)).unwrap();
	let texture_1 = texture_creator.create_texture_from_surface(&surface_1).unwrap();
	canvas.copy(&texture_1, None, sdl2::rect::Rect::new(screen_size[0] / 2 - 200, 25, 50 * score_1.to_string().len() as u32, 100)).unwrap();

	let surface_2 = font.render(&score_2.to_string()).blended(Color::RGBA(0, 255, 0, 255)).unwrap();
	let texture_2 = texture_creator.create_texture_from_surface(&surface_2).unwrap();
	canvas.copy(&texture_2, None, sdl2::rect::Rect::new(screen_size[0] / 2 + 200, 25, 50 * score_2.to_string().len() as u32, 100)).unwrap();

}