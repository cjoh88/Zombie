//use sfml::graphics::{Drawable, RenderStates, RenderTarget, RectangleShape, Color};
use sfml::system::{Vector2f};
//use sfml::graphics::*;
use sfml::graphics::{Drawable, RenderStates, RenderTarget, RectangleShape, Color, VertexArray, Texture, Quads};
use rustc_serialize::json;
use std::fs::File;
use std::io::Read;
use util;
//use self::util::*;
use game::map_generator::TerrainGenerator;
use std::slice::{Iter, IterMut};
use std::ops::{Index, IndexMut};

pub struct TileArray(Vec<u32>);

impl TileArray {
	pub fn new() -> Self {
		TileArray(Vec::new())
	}

	pub fn with_capacity(capacity: usize) -> Self {
		TileArray(Vec::with_capacity(capacity))
	}

	pub fn iter(&self) -> Iter<u32> {
		self.iter()
	}

	pub fn iter_mut(&mut self) -> IterMut<u32> {
		self.iter_mut()
	}
}



pub struct Layer {
	vertices: VertexArray,
	texture: Texture,
	tiles: Vec<u32>,
}

impl Layer {
	//TODO Use Map settings struct instead of several parameters
	pub fn new(width: usize, height: usize, tiles: Vec<u32>) -> Self {
		let size = width * height;
		let texture = Texture::new_from_file("assets/tileset.png").expect("Could not load tileset");
		//let tiles = vec![0; size];		//TODO Load from file
		let mut vertices = VertexArray::new_init(Quads, size as u32 * 4).expect("Could not create VertexArray");
		for (i, tile) in tiles.iter().enumerate() {
			//let tx = ((*tile * 64) % 640) as f32;		// texture width = 640
			//let ty = ((*tile * 64) / 640) as f32;
			let (tx0, ty0, tx1, ty1) = util::get_tile_texture_coords(*tile);

			let px = (i % width) as f32;
			let py = (i / width) as f32;

			let j = i as u32;
			vertices.get_vertex(j * 4).0.position = Vector2f::new(px, py);
			vertices.get_vertex(j * 4 + 1).0.position = Vector2f::new(px + 1.0, py);			// in game tile size = 1.0
			vertices.get_vertex(j * 4 + 2).0.position = Vector2f::new(px + 1.0, py + 1.0);
			vertices.get_vertex(j * 4 + 3).0.position = Vector2f::new(px, py + 1.0);

			vertices.get_vertex(j * 4).0.tex_coords = Vector2f::new(tx0, ty0);	
			vertices.get_vertex(j * 4 + 1).0.tex_coords = Vector2f::new(tx1, ty0);			// texture tile size = 64.0
			vertices.get_vertex(j * 4 + 2).0.tex_coords = Vector2f::new(tx1, ty1);
			vertices.get_vertex(j * 4 + 3).0.tex_coords = Vector2f::new(tx0, ty1);
		
			//vertices.get_vertex(j * 4).0.tex_coords = Vector2f::new(tx, ty);	
			//vertices.get_vertex(j * 4 + 1).0.tex_coords = Vector2f::new(tx + 64.0, ty);			// texture tile size = 64.0
			//vertices.get_vertex(j * 4 + 2).0.tex_coords = Vector2f::new(tx + 64.0, ty + 64.0);
			//vertices.get_vertex(j * 4 + 3).0.tex_coords = Vector2f::new(tx, ty + 64.0);
		}
		Layer {
			vertices: vertices,
			texture: texture,
			tiles: tiles,
		}
	}
}

impl Drawable for Layer {
	fn draw<RT: RenderTarget>(&self, target: &mut RT, _: &mut RenderStates) {
		let mut rs = RenderStates::default();	//TODO include RenderStates in struct?
		rs.texture = Some(&self.texture);
		//target.draw_vertex_array_rs(&self.vertices, &mut rs);
		target.draw_vertex_array(&self.vertices, &mut rs);
	}
}


pub struct Map {
	width: usize,
	height: usize,
	layer: [Layer; 3],
}


impl Map {
	pub fn new_default() -> Self {
		let width = 128;
		let height = 128;
		Map {
			width: width,
			height: height,
			layer: {
				//let x = TerrainGenerator::new_from_seed(&[1,2,3,4], 128,128);
				let x = TerrainGenerator::new_from_seed(&[5,6,7,8], 128,128);
				[x, Layer::new(width, height, vec![0; width*height]), Layer::new(width, height, vec![0; width*height])]
			},
		}
	}

	pub fn new_init(width: usize, height: usize, l0: Vec<u32>, l1: Vec<u32>, l2: Vec<u32>) -> Self {
		Map {
			width: width,
			height: height,
			layer: [Layer::new(width, height, l0),Layer::new(width, height, l1),Layer::new(width, height, l2)],
		}
	}

	pub fn get_layer(&self, n: usize) -> &Layer {
		&self.layer[n]
	}
}

impl Drawable for Map {
	fn draw<RT: RenderTarget>(&self, render_target: &mut RT, _: &mut RenderStates) {
		//for i in self.layer[0].iter() {
			//self.rect.set_fill_color(&self.tile_info[i].color);
			//self.rect.set_fill_color(&self.tile_info[i].color);
			//render_target.draw(&self.rect);
		//}
		render_target.draw(&self.layer[0]);
		render_target.draw(&self.layer[1]);
		render_target.draw(&self.layer[2]);
	}
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct MapLoader {
	name: String,
	description: String,
	width: u32,
	height: u32,
	tileset0: String,
	tileset1: String,
	tileset2: String,
	layer0: Vec<u32>,
	layer1: Vec<u32>,
	layer2: Vec<u32>,
}

impl MapLoader {
	pub fn test() {
		let map_loader = MapLoader {
			name: "Test map".to_string(),
			description: "A test map".to_string(),
			width: 64,
			height: 64,
			tileset0: "tileset.png".to_string(),
			tileset1: "tileset.png".to_string(),
			tileset2: "tileset.png".to_string(),
			layer0: vec![0;64*64],
			layer1: vec![0;64*64],
			layer2: vec![0;64*64],
		};
		let encoded = json::encode(&map_loader).unwrap();
		println!("{}", encoded);
	}

	pub fn load(file: &str) -> Map {
		let folder = "assets/".to_string();
		let mut file = File::open(folder + file).expect("Could not open file!");
		let mut buf = String::new();
		file.read_to_string(&mut buf).expect("Could not read file!");
		let decoded: MapLoader = json::decode(&buf).expect("Could not decode");
		Map::new_init(decoded.width as usize, decoded.height as usize, decoded.layer0, decoded.layer1, decoded.layer2)
	}
}
