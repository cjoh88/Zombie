use game::camera::Camera;
use game::map::{Map, Layer, MapLoader};
use sfml::graphics::{Drawable, RenderStates, RenderTarget, RectangleShape, Color};
use sfml::system::{Vector2f};
use game::map_generator::{Noise, NoiseRenderer};

pub struct World {
	camera: Camera,
	map: Map,
	noise_renderer: NoiseRenderer,
}


impl World {
	pub fn new() -> Self {
		World {
			camera: Camera::new(64f32, 36f32, 50f32),
			map: MapLoader::load("testmap2.json"),
			noise_renderer: {
				let x = Noise::new_perlin_from_seed(&[4,5,6,7], 64, 64, 6, 0.8);
				NoiseRenderer::new(&x)
			}
		}
	}

	pub fn new_empty() -> Self {
		World {
			camera: Camera::new(64f32, 36f32, 50f32),
			map: Map::new_default(),
			noise_renderer: {
				let x = Noise::new_perlin_from_seed(&[1,2,3,4], 64, 64, 6, 0.5);
				NoiseRenderer::new(&x)
			}
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
		//render_target.draw(&self.noise_renderer)
	}
}