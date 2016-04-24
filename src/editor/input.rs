use sfml::window::{Key};
use sfml::window::{event};
use sfml::window::MouseButton;
use sfml::graphics::{RenderWindow};
use sfml::system::{Vector2i, Vector2f};
use sfml::graphics::RenderTarget;

use editor::world::World;

pub struct EditorInputHandler {
	dummy: i32
}

impl EditorInputHandler {
	pub fn new() -> Self {
		EditorInputHandler {
			dummy: 0,
		}
	}

	pub fn handle_input(&mut self, world: &mut World, window: &mut RenderWindow) {
		for event in window.events() {
			match event {
				event::Closed => window.close(),
				event::KeyPressed{code, ..} => self.handle_key_pressed(world, window, code),	//world, window, code
				event::KeyReleased{code, ..} => self.handle_key_released(world, window, code),	//world, window, code
				event::MouseButtonPressed{button, x, y} => self.handle_mouse_pressed(world, window, button, x, y), //world, window, button, x, y
				_ => (),
			}
		}
	}

	fn handle_key_pressed(&mut self, world: &mut World, window: &mut RenderWindow, code: Key) {
		match code {
            Key::Escape => window.close(),
            Key::Right | Key::D  => world.get_mut_camera().move_right(),
            Key::Left | Key::A  => world.get_mut_camera().move_left(),
            Key::Up | Key::W  => world.get_mut_camera().move_up(),
            Key::Down | Key::S  => world.get_mut_camera().move_down(),
            _   => ()
        }
	}
	fn handle_key_released(&mut self, world: &mut World, window: &mut RenderWindow, code: Key) {
        match code {
            Key::Right | Key::Left | Key::D | Key::A => world.get_mut_camera().stop_horizontal(),
            Key::Up | Key::Down | Key::W | Key::S    => world.get_mut_camera().stop_vertical(),
            Key::Add               => world.get_mut_camera().zoom(0.5),
            Key::Subtract          => world.get_mut_camera().zoom(2.0),
            _ => ()
        }
    }

    fn handle_mouse_pressed(&mut self, editor: &mut World, window: &mut RenderWindow, code: MouseButton, x: i32, y: i32) {
        /*let v = Vector2i::new(x,y);
        let v2: Vector2f = window.map_pixel_to_coords(&v, &editor.get_view());
        let v3 = screen_to_map(v2);
        println!("Mouse({}, {})", v3.x, v3.y);*/

    }
}