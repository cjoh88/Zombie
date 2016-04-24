use sfml::system::{Vector2f};
use sfml::graphics::{View};
//use self::sfml::graphics::{Shape, Transformable, RenderWindow, Color, RectangleShape, RenderTarget, Vertex, VertexArray, PrimitiveType};



pub struct Camera {
	view: View,
	velocity: Vector2f,
	speed: f32,
}

impl Camera {
	pub fn new(width: f32, height: f32, speed: f32) -> Self {
		Camera {
			view: View::new_init(
				&Vector2f::new(width / 2.0, height / 2.0), 
				&Vector2f::new(width, height))
				.expect("Could not create View"),
			velocity: Vector2f::new(0.0, 0.0),
			speed: speed,
		}
	}

	pub fn move_right(&mut self) {
		self.velocity.x = self.speed;
	}

	pub fn move_left(&mut self) {
		self.velocity.x = -self.speed;
	}

	pub fn move_up(&mut self) {
		self.velocity.y = -self.speed;
	}

	pub fn move_down(&mut self) {
		self.velocity.y = self.speed;
	}

	pub fn stop_horizontal(&mut self) {
		self.velocity.x = 0.0;
	}

	pub fn stop_vertical(&mut self) {
		self.velocity.y = 0.0;
	}

	pub fn zoom(&mut self, amount: f32) {
		self.view.zoom(amount);
	}

	pub fn update(&mut self, delta: f32) {
		self.view.move_(&(self.velocity * delta));
	}

	pub fn get_view(&self) -> &View {
		&self.view
	}
}