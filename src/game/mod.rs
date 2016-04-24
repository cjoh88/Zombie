pub mod input;
pub mod world;
pub mod camera;
pub mod map;
pub mod map_generator;

use sfml::graphics::{RenderWindow, Color, RenderTarget, View};
use sfml::window::{VideoMode, ContextSettings, window_style, event};
//use sfml::window::keyboard::{Key};
use sfml::system::{Vector2f, Clock};
use game::world::World;
/*use game::world_renderer::WorldRenderer;
use game::input::PlayerInputHandler;
use game::map_generator::MapGenerator;
use util::noise::{Noise2D, Noise2DGenerator, NoiseType};
use util::grid::Grid;*/

use game::input::PlayerInputHandler;

pub struct Game {
	window: RenderWindow,
	world: World,
	player_input_handler: PlayerInputHandler,
}

impl Game {
	pub fn new(width: u32, height: u32, name: &'static str, fullscreen: bool) -> Self {
		let settings: ContextSettings = ContextSettings::default();
		let style = match fullscreen {
			true => window_style::FULLSCREEN,
			false => window_style::CLOSE | window_style::RESIZE,
		};
		let video_mode = VideoMode::new_init(width, height, 32);
		let window = RenderWindow::new(video_mode, name, style, &settings).expect("Could not create RenderWindow");

		let mut game =  Game {
			window: window,
			world: World::new_empty(),
			player_input_handler: PlayerInputHandler::new(),
		};
		game.window.set_vertical_sync_enabled(false);
		game.window.set_framerate_limit(60);

		game
	}

	pub fn run(&mut self) {
		let mut clock = Clock::new();
		let mut delta = 0.0;
		while self.window.is_open() {
			self.handle_input();
			self.update(delta);
			//println!("FPS: {}", 1.0/delta);
			self.window.set_view(self.world.get_camera().get_view());
			self.render();
			delta = clock.restart().as_seconds();
		}
	}

	fn handle_input(&mut self) {
		self.player_input_handler.handle_input(&mut self.world, &mut self.window); //TODO Replace self with world
	}

	fn render(&mut self) {
		self.window.clear(&Color::black());
		self.window.draw(&self.world);
		//TODO Render game objects
		self.window.display();
	}

	fn update(&mut self, delta: f32) {
		self.world.update(delta);
	}
}