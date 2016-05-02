extern crate sfml;
extern crate rand;
extern crate rustc_serialize;

mod game;
mod editor;
mod util;

use game::Game;
use editor::Editor;

//use game::map::MapLoader;
//use game::map_generator::MapGenerator;
use game::map_generator::RoadGenerator;

//use util::Vec2D;



fn main() {

	let mut x = RoadGenerator::new(&[1,2,3,4], 10, 10, 3);
	x.generate();
	//x.print_roads();

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
