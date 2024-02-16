#[derive(PartialEq, Debug)]
pub enum Direction {
	Up,
	Down,
	Stationary,
}

pub struct Bat {
	position: [i32; 2],
	direction: Direction,
	colour: sdl2::pixels::Color,
}

impl Bat {
	pub fn new(position: [i32; 2], colour: sdl2::pixels::Color) -> Bat {
		Bat {
			position,
			direction: Direction::Stationary,
			colour,
		}
	}

	pub fn change_direction(&mut self, direction: Direction) {
		self.direction = direction;
	}

	pub fn move_bat(&mut self) {
		match self.direction {
			Direction::Up => self.position[1] -= 1,
			Direction::Down => self.position[1] += 1,
			Direction::Stationary => (),
		}
	}

	pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
		canvas.set_draw_color(self.colour);
		let bat_rect = sdl2::rect::Rect::new(self.position[0], self.position[1], 25, 100);
		canvas.fill_rect(bat_rect).unwrap();
	}

	pub fn get_position(&self) -> [i32; 2] {
		self.position
	}
	pub fn get_colour(&self) -> sdl2::pixels::Color {
		self.colour
	}
}