extern crate sdl2;

const SCREEN_SIZE: [i32; 2] = [1280, 720];

fn main() {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem.window("Snake Game", SCREEN_SIZE[0] as u32, SCREEN_SIZE[1] as u32)
		.position_centered()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().build().unwrap();
	canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
	canvas.clear();
	canvas.present();

	let mut event_pump = sdl_context.event_pump().unwrap();

	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				sdl2::event::Event::Quit {..} => {
					break 'running
				},
				_ => {}
			}
		}
	}
}