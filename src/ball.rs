#[derive(PartialEq, Debug)]
pub enum Collision {
	Bat,
	TopBottom,
	Side,
	None
}

pub struct Ball {
	position: [i32; 2],
	velocity: [i32; 2],
	collision: Collision,
	colour: sdl2::pixels::Color,
}

impl Ball {
	pub fn new(position: [i32; 2], velocity: [i32; 2], colour: sdl2::pixels::Color) -> Ball {
		Ball {
			position,
			velocity,
			collision: Collision::None,
			colour,
		}
	}

	pub fn move_ball(&mut self) {
		self.position[0] += self.velocity[0];
		self.position[1] += self.velocity[1];
	}

	pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
		canvas.set_draw_color(self.colour);
		let ball_rect = sdl2::rect::Rect::new(self.position[0], self.position[1], 25, 25);
		canvas.fill_rect(ball_rect).unwrap();
	}

}