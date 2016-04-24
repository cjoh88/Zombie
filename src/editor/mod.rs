pub mod input;
pub mod world;

use sfml::graphics::{RenderWindow, Color, RenderTarget, View};
use sfml::window::{VideoMode, ContextSettings, window_style, event};
use sfml::system::{Vector2f, Clock};

use editor::world::World;
use game::camera::Camera;
use editor::input::EditorInputHandler;

pub struct Editor {
	window: RenderWindow,
	world: World,
	input: EditorInputHandler,
}

impl Editor {
	pub fn new(width: u32, height: u32) -> Self {
		let settings = ContextSettings::default();
		let style = window_style::CLOSE | window_style::RESIZE;
		let video_mode = VideoMode::new_init(width, height, 32);
		let window = RenderWindow::new(video_mode, "Editor", style, &settings).expect("Could not create RenderWindow");
		let mut editor = Editor {
			window: window,
			world: World::new_empty(),
			input: EditorInputHandler::new(),
		};
		editor.window.set_vertical_sync_enabled(false);
		editor.window.set_framerate_limit(60);
		editor
	}

	pub fn run(&mut self) {
		let mut clock = Clock::new();
		let mut delta = 0.0;
		while self.window.is_open() {
			self.handle_input();
			self.update(delta);
			self.window.set_view(self.world.get_camera().get_view());
			self.render();
			delta = clock.restart().as_seconds();
		}
	}

	fn handle_input(&mut self) {
		self.input.handle_input(&mut self.world, &mut self.window);
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