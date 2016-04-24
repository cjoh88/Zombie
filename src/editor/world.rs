use game::camera::Camera;
use game::map::{Map, Layer, MapLoader};
use sfml::graphics::{Drawable, RenderStates, RenderTarget, RectangleShape, Color};
use sfml::system::{Vector2f};

pub struct World {
	camera: Camera,
	map: Map,
}


impl World {
	pub fn new() -> Self {
		World {
			camera: Camera::new(64f32, 36f32, 50f32),
			map: MapLoader::load("testmap2.json"),
		}
	}

	pub fn new_empty() -> Self {
		World {
			camera: Camera::new(64f32, 36f32, 50f32),
			map: Map::new_default(),
		}
	}

	pub fn update(&mut self, delta: f32) {
		self.camera.update(delta);
	}

	pub fn get_mut_camera(&mut self) -> &mut Camera {
		&mut self.camera
	}

	pub fn get_camera(&self) -> &Camera {
		&self.camera
	}
}

impl Drawable for World {
	fn draw<RT: RenderTarget>(&self, render_target: &mut RT, _: &mut RenderStates) {
		//render_target.draw(&self.test);
		//render_target.draw(&self.map);
		render_target.draw(self.map.get_layer(0));
	}
}