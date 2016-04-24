extern crate sfml;
extern crate rand;
extern crate rustc_serialize;

mod game;
mod editor;
mod util;

use game::Game;
use editor::Editor;

use game::map::MapLoader;
use game::map_generator::MapGenerator;

use util::Vec2D;

fn test1(x: &Vec2D<u32>) {
	println!("{}, {}", x[(0,0)], x[(1,0)]);
}

fn test2(x: &Vec<u32>) {
	println!("{}, {}", x[0], x[1]);
}

fn main() {

	let x: Vec2D<u32> = {
		let mut y =	Vec2D::new(2,1);
		(*y).push(3);
		(*y).push(4);
		y
	};
	test1(&x);
	test2(&x);

	/*for c in CoordIterator::new(10,5) {
		println!("{}, {}", c.0, c.1);
	}

	let seed: &[usize] = &[1,2,3,4];
	let mut x = MapGenerator::new_seed(seed);
	for i in 0..100{
		println!("{}", x.next_usize() % 25);
	}

	panic!("BANANA");*/

	let start_editor = false;

	if start_editor {
		let mut editor = Editor::new(1600, 900);
		editor.run();
	}
	else {
    	let mut game = Game::new(1600,900, "Game", false);
    	game.run();
	}
    //MapLoader::test();
    //MapLoader::load();
}
