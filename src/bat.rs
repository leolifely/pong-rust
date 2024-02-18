#[derive(PartialEq, Debug)]
pub enum Direction {
	Up,
	Down,
	Stationary,
}

pub struct Bat {
	position: [i32; 2],
	size: [i32; 2],
	score: i32,
	direction: Direction,
	colour: sdl2::pixels::Color,
}

impl Bat {
	pub fn new(position: [i32; 2], size: [i32; 2], colour: sdl2::pixels::Color) -> Bat {
		Bat {
			position,
			size,
			score: 0,
			direction: Direction::Stationary,
			colour,
		}
	}

	pub fn change_direction(&mut self, direction: Direction) {
		self.direction = direction;
	}

	pub fn move_bat(&mut self, screen_size: [i32; 2]) {
		// Stop the bat from moving off the screen
		if self.position[1] <= 0 && self.direction == Direction::Up {
			self.direction = Direction::Stationary;
		}
		if self.position[1] >= screen_size[1] - self.size[1] && self.direction == Direction::Down {
			self.direction = Direction::Stationary;
		}
		// Move the bat depending on self.direction
		match self.direction {
			Direction::Up => self.position[1] -= 2,
			Direction::Down => self.position[1] += 2,
			Direction::Stationary => (),
		}
	}

	pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
		canvas.set_draw_color(self.colour);
		// Create a rectangle and fill it with the bat's colour
		let bat_rect = sdl2::rect::Rect::new(self.position[0], self.position[1], self.size[0] as u32, self.size[1] as u32);
		canvas.fill_rect(bat_rect).unwrap();
	}

	pub fn get_position(&self) -> [i32; 2] {
		self.position
	}
	pub fn get_colour(&self) -> sdl2::pixels::Color {
		self.colour
	}
	pub fn get_size(&self) -> [i32; 2] {
		self.size
	}
	pub fn get_score(&self) -> i32 {
		self.score
	}
	pub fn increase_score(&mut self) {
		self.score += 1;
	}
}