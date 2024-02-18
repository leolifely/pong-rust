#[derive(PartialEq, Debug)]
pub enum Collision {
	Bat,
	TopBottom,
	LeftSide,
	RightSide,
	None,
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
		// Add the velocity to the position
		self.position[0] += self.velocity[0];
		self.position[1] += self.velocity[1];
	}

	pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
		canvas.set_draw_color(self.colour);
		// Create a rectangle and fill it with the ball's colour
		let ball_rect = sdl2::rect::Rect::new(self.position[0], self.position[1], 25, 25);
		canvas.fill_rect(ball_rect).unwrap();
	}

	pub fn test_collision(&mut self, bat_1: &crate::bat::Bat, bat_2: &crate::bat::Bat, screen_size: [i32; 2]) -> Collision {
		// Check for collisions with the top and bottom of the screen
		let ball_size: i32 = 25;
		let ball_radius: i32 = ball_size / 2;
		if self.position[1] <= 0 || self.position[1] >= screen_size[1] - ball_size {
			self.velocity[1] = -self.velocity[1];
			return Collision::TopBottom;
		}
		// Check for collisions with the bats
		if self.position[0] <= bat_1.get_position()[0] + bat_1.get_size()[0] + ball_radius && self.position[1] >=
			bat_1.get_position()[1] && self.position[1] <= bat_1.get_position()[1] + bat_1.get_size()[1] {
			if self.velocity[0] < 0 { // Only change direction if the ball is moving towards the bat
				self.velocity[0] = -self.velocity[0];
				return Collision::Bat;
			}
		}
		if self.position[0] >= bat_2.get_position()[0] - bat_2.get_size()[0] - ball_radius && self.position[1] >=
			bat_2.get_position()[1] && self.position[1] <= bat_2.get_position()[1] + bat_2.get_size()[1] {
			if self.velocity[0] > 0 { // Only change direction if the ball is moving towards the bat
				self.velocity[0] = -self.velocity[0];
				return Collision::Bat;
			}
		}
		// Check for collisions with the sides of the screen
		if self.position[0] <= 0 {
			self.position = [screen_size[0] / 2, screen_size[1] / 2];
			self.velocity = [1, 1];
			return Collision::LeftSide;
		}
		if self.position[0] >= screen_size[0] - ball_size {
			self.position = [screen_size[0] / 2, screen_size[1] / 2];
			self.velocity = [-1, -1];
			return Collision::RightSide;
		}
		Collision::None
	}

}