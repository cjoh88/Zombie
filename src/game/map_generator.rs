
use game::map::{Map, Layer, CollisionLayer};
use rand::{Rng, SeedableRng, StdRng};
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};
use sfml::system::{Vector2f, Vector2u};
//use sfml::graphics::*;
use sfml::graphics::{Drawable, RenderStates, RenderTarget, RectangleShape, Color, VertexArray, Texture, Quads, IntRect};
use util;
use util::Vec2D;

pub struct MapGenerator {
	rng: StdRng,
}

impl MapGenerator {
	pub fn new_seed(seed: &[usize]) -> Self{
		MapGenerator {
			rng: SeedableRng::from_seed(seed)
		}
	}

	pub fn next_usize(&mut self) -> usize{
		self.rng.gen::<usize>()
	}

	pub fn next_f32(&mut self) -> f32 {
		self.rng.gen::<f32>()
	}
}

pub struct Noise(Vec2D<f32>);

impl Noise {
	fn new(width: usize, height: usize) -> Self {
		Noise(Vec2D::from_vec(width, height, vec![0f32; width * height]))
	}

	pub fn new_white_from_seed(seed: &[usize], width: usize, height: usize) -> Self {
		let mut rng: StdRng = SeedableRng::from_seed(seed);
		let mut vec: Vec<f32> = Vec::with_capacity(width * height);
		for i in 0..(width * height) {
					vec.push(rng.gen::<f32>());
		}
		Noise(Vec2D::from_vec(width, height, vec))
	}

	pub fn new_smooth_from_seed(seed: &[usize], width: usize, height: usize, octave: usize) -> Self {
		let base_noise = Noise::new_white_from_seed(seed, width, height);
		let mut smooth_noise = Noise::new(width, height);
		let sample_period = 1 << octave;
		let sample_frequency = 1f32 / sample_period as f32;
		for x in 0..width {
			let x0 = (x / sample_period) * sample_period;
			let x1 = (x0 + sample_period) % width;
			let horizontal_blend = (x - x0) as f32 * sample_frequency;
			for y in 0..height {
				let y0 = (y / sample_period) * sample_period;
				let y1 = (y0 + sample_period) % height;
				let vertical_blend = (y - y0) as f32 * sample_frequency;

				let top = Self::interpolate(base_noise[(x0,y0)], base_noise[(x1,y0)], horizontal_blend);
				let bottom = Self::interpolate(base_noise[(x0,y1)], base_noise[(x1,y1)], horizontal_blend);
				smooth_noise[(x,y)] = Self::interpolate(top, bottom, vertical_blend);
			}
		}
		smooth_noise
	}

	pub fn new_smooth_from_white(base_noise: &Noise, octave: usize) -> Self {
		let mut smooth_noise = Noise::new(base_noise.width(), base_noise.height());
		let sample_period = 1 << octave;
		let sample_frequency = 1f32 / sample_period as f32;
		for x in 0..base_noise.width() {
			let x0 = (x / sample_period) * sample_period;
			let x1 = (x0 + sample_period) % base_noise.width();
			let horizontal_blend = (x - x0) as f32 * sample_frequency;
			for y in 0..base_noise.height() {
				let y0 = (y / sample_period) * sample_period;
				let y1 = (y0 + sample_period) % base_noise.height();
				let vertical_blend = (y - y0) as f32 * sample_frequency;

				let top = Self::interpolate(base_noise[(x0,y0)], base_noise[(x1,y0)], horizontal_blend);
				let bottom = Self::interpolate(base_noise[(x0,y1)], base_noise[(x1,y1)], horizontal_blend);
				smooth_noise[(x,y)] = Self::interpolate(top, bottom, vertical_blend);
			}
		}
		smooth_noise
	}

	pub fn new_perlin_from_seed(seed: &[usize], width: usize, height: usize, octave_count: usize, persistance: f32) -> Self {
		let white_noise = Noise::new_white_from_seed(seed, width, height);
		let mut perlin_noise = Noise::new(width, height);
		let mut amplitude = 1f32;
		let mut total_amplitude = 0f32;
		for octave in 0..octave_count {
			amplitude *= persistance;
			total_amplitude += amplitude;
			perlin_noise.add_amp(&Noise::new_smooth_from_white(&white_noise, octave_count - octave), amplitude);
		}
		perlin_noise.normalize_amp(total_amplitude);
		perlin_noise
	}

	pub fn interpolate(x0: f32, x1: f32, alpha: f32) -> f32 {
		x0 * (1f32 - alpha) + alpha * x1
	}

	pub fn add(&mut self, other: &Noise) {
		for (x, y) in self.iter_mut().zip(other.iter()) {
			*x = *x + *y;
		}
	}

	pub fn add_amp(&mut self, other: &Noise, amplitude: f32) {
		for (x, y) in self.iter_mut().zip(other.iter()) {
			*x = *x + (*y * amplitude);
		}
	}

	pub fn normalize_amp(&mut self, total_amplitude: f32) {
		for x in self.iter_mut() {
			*x = *x / total_amplitude;
		}
	}

	pub fn iter(&self) -> Iter<f32> {
		self.0.iter()
	}

	pub fn iter_mut(&mut self) -> IterMut<f32> {
		self.0.iter_mut()
	}

	pub fn width(&self) -> usize {
		self.0.width()
	}

	pub fn height(&self) -> usize {
		self.0.height()
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}
}

impl Index<(usize, usize)> for Noise {
	type Output = f32;
	fn index(&self, p: (usize, usize)) -> &f32 {
		&self.0[p]
	}
}

impl IndexMut<(usize, usize)> for Noise {
	fn index_mut(&mut self, p: (usize, usize)) -> &mut f32 {
		&mut self.0[p]
	}
}

pub struct NoiseRenderer {
	vertices: VertexArray
}

impl NoiseRenderer {
	pub fn new(noise: &Noise) -> Self {
		let size = noise.len();
		let mut vertices = VertexArray::new_init(Quads, size as u32 * 4).expect("Could not create VertexArray");
		for (i, x) in noise.iter().enumerate() {

			let px = (i % noise.width()) as f32;
			let py = (i / noise.width()) as f32;

			let j = i as u32;
			vertices.get_vertex(j * 4).0.position = Vector2f::new(px, py);
			vertices.get_vertex(j * 4 + 1).0.position = Vector2f::new(px + 1.0, py);			// in game tile size = 1.0
			vertices.get_vertex(j * 4 + 2).0.position = Vector2f::new(px + 1.0, py + 1.0);
			vertices.get_vertex(j * 4 + 3).0.position = Vector2f::new(px, py + 1.0);

			vertices.get_vertex(j * 4).0.color.blue = (*x * 255f32) as u8; 
			vertices.get_vertex(j * 4 + 1).0.color.blue = (*x * 255f32) as u8; 
			vertices.get_vertex(j * 4 + 2).0.color.blue = (*x * 255f32) as u8; 
			vertices.get_vertex(j * 4 + 3).0.color.blue = (*x * 255f32) as u8; 
		}
		NoiseRenderer {
			vertices: vertices,
		}
	}
}

impl Drawable for NoiseRenderer {
	fn draw<RT: RenderTarget>(&self, target: &mut RT, _: &mut RenderStates) {
		let mut rs = RenderStates::default();	//TODO include RenderStates in struct?
		//rs.texture = Some(&self.texture);
		target.draw_vertex_array(&self.vertices, &mut rs);
	}
}


const Grass: u32 = 0u32;
const ShallowWater: u32 = 1u32;
const Road: u32 = 2u32;
const Snow: u32 = 3u32;
const DeepWater: u32 = 4u32;
const Forest: u32 = 5u32;
const Dirt: u32 = 6u32;
const Sand: u32 = 7u32;
const Swamp: u32 = 8u32;
const Building: u32 = 9u32;

const oct: usize = 6;
const per: f32 = 0.6;

pub struct TerrainGenerator {
	dummy: i32,
}

impl TerrainGenerator {
	pub fn new_from_seed(seed: &[usize], width: usize, height: usize) -> Layer {
		let noise = Noise::new_perlin_from_seed(seed, width, height, oct, per);
		//let mut tiles: Vec<u32> = Vec::with_capacity(width * height);
		let mut tiles: Vec2D<u32> = Vec2D::new(width, height);
		for n in noise.iter() {
			let mut x = match *n {
				b if b < 0.05 => Snow,
				b if b < 0.15 => Road,
				b if b < 0.5 => Grass,
				b if b < 0.65 => Forest,
				_ => DeepWater,
			};
			(*tiles).push(x);
		}
		let mut t = Self::smooth_terrain(&tiles);
		for _ in 0..0 {
			t = Self::smooth_terrain(&t);
		}
		//t[(0,0)] = Sand;
		Layer::new(t)
	}

	fn smooth_terrain(tiles: &Vec2D<u32>) -> Vec2D<u32> {
		let mut new: Vec2D<u32> = Vec2D::new(tiles.width(), tiles.height());
		//let (x, y) = util::itoc(3+ 256, 128);
		//let i = util::ctoi(x,y,128);
		//println!("{}, {} = {}", x, y, i);
		for (i, tile) in tiles.iter().enumerate() {
			let (x,y) = util::itoc(i, 128);
			if x > 0 && x < 127 && y > 0 && y < 127 {
				(*new).push(Self::smooth_tile(
					tiles[(x,y)],
					tiles[(x,y+1)],
					tiles[(x+1,y)],
					tiles[(x,y-1)],
					tiles[(x-1,y)],
				));
			}
			else {
				(*new).push(tiles[(x,y)]);
			}
		}
		//TODO fix this, this is just for testing
		Self::add_roads(&mut new);
		new
	}

	fn add_roads(tiles: &mut Vec2D<u32>) {
		let mut road_generator = RoadGenerator::new(&[1,2,3,4], 128, 128, 5);	//TODO Change seed
		road_generator.generate();
		//road_generator.print_roads();
		for road in road_generator.roads.iter() {
			if road.x1 == road.x2 {		// if vertical
				for y in road.y1..road.y2 {
					tiles[(road.x1, y)] = Road;
				}
			}
			else {	// if horizontal
				for x in road.x1..road.x2 {
					tiles[(x, road.y1)] = Road;
				}
			}
		}
	}

	//TODO Do something about this.... OMFG
	fn smooth_tile(curr: u32, top: u32, right: u32, bottom: u32, left: u32) -> u32 {
		let mut t = 0;
		if top == right {
			t += 1;
		}
		if top == bottom {
			t += 1;
		}
		if top == left {
			t += 1;
		}
		if t >= 3 {
			return top;
		}
		t = 0;
		if right == top {
			t += 1;
		}
		if right == bottom {
			t += 1;
		}
		if right == left {
			t += 1;
		}
		if t >= 3 {
			return right;
		}
		t = 0;
		if bottom == top {
			t += 1;
		}
		if bottom == right {
			t += 1;
		}
		if bottom == left {
			t += 1;
		}
		if t >= 3 {
			return bottom;
		}
		t = 0;
		if left == top {
			t += 1;
		}
		if left == right {
			t += 1;
		}
		if left == bottom {
			t += 1;
		}
		if t >= 3 {
			return left;
		}
		return curr;
	}
}

struct Rect {
	x1: usize,
	y1: usize,
	x2: usize,
	y2: usize,
}

impl Rect {
	fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
		Rect {
			x1: x1,
			y1: y1,
			x2: x2,
			y2: y2,
		}
	}

	fn width(&self) -> usize {
		self.x2 - self.x1
	}

	fn height(&self) -> usize {
		self.y2 - self.y1
	}

	fn split_vertical(&self, x: usize) -> (Rect, Rect) {
		let left = Rect {
			x1: self.x1,
			y1: self.y1,
			x2: self.x1+x,
			y2: self.y2,
		};
		let right = Rect {
			x1: self.x1+x+1,
			y1: self.y1,
			x2: self.x2,
			y2: self.y2,
		};
		(left, right)
	}

	fn split_horizontal(&self, y: usize) -> (Rect, Rect) {
		let top = Rect {
			x1: self.x1,
			y1: self.y1,
			x2: self.x2,
			y2: self.y1+y,
		};
		let bottom = Rect {
			x1: self.x1,
			y1: self.y1+y+1,
			x2: self.x2,
			y2: self.y2,
		};
		(top, bottom)
	}

	fn print(&self) {
		println!("{}, {}, {}, {}", self.x1, self.y1, self.x2, self.y2);
	}
}

pub struct RoadGenerator {
	width: usize,
	height: usize,
	rng: StdRng,
	roads: Vec<Rect>,
	splits: usize,
}

impl RoadGenerator {
	pub fn new(seed: &[usize], width: usize, height: usize, splits: usize) -> Self {
		RoadGenerator {
			width: width,
			height: height,
			rng: SeedableRng::from_seed(seed),
			roads: Vec::new(),
			splits: splits,
		}
	}

	//TODO Don't generate roads of length 0
	pub fn generate(&mut self) {
		let size_limit: usize = 12; // min size of a split
		//TODO empty roads
		let mut process_list: Vec<(bool, Rect)> = Vec::new();
		process_list.push((self.rng.gen::<bool>(), Rect{x1: 0, y1: 0, x2: self.width, y2: self.height}));
		while !process_list.is_empty() {
			let (vertical, rect) = process_list.pop().expect("Could not pop vector");
			if vertical && rect.width() >= size_limit {
				let mut x = self.rng.gen::<usize>() % (rect.width()-8) + 4 as usize;
				let (left, right): (Rect, Rect) = rect.split_vertical(x);
				process_list.push((false, left));
				process_list.push((false, right));
				self.roads.push(Rect::new(rect.x1+x, rect.y1, rect.x1+x, rect.y2));
			}
			else if rect.height() >= size_limit {
				let mut y = self.rng.gen::<usize>() % (rect.height()-8) + 4 as usize;
				let (top, bottom): (Rect, Rect) = rect.split_horizontal(y);
				process_list.push((true, top));
				process_list.push((true, bottom));
				self.roads.push(Rect::new(rect.x1, rect.y1+y, rect.x2, rect.y1+y));
			}
		}
	}

	pub fn print_roads(&self) {
		for road in self.roads.iter() {
			if road.x1 == road.x2 {
				println!("vertical  ({}): {},{}", road.x1, road.y1, road.y2);
			}
			else {
				println!("horizontal({}): {}, {}", road.y1,road.x1, road.x2);
			}
		}
	}
}	

pub struct Building {
	position: Vector2u,
	tiles: Vec2D<u32>,	
	collision: CollisionLayer,	
}

impl Building {
	pub fn new(x: u32, y: u32, width: usize, height: usize) -> Self {
		let mut tiles: Vec<u32> = Vec::with_capacity(width * height);
		for _ in 0..(width * height) {
			tiles.push(Building);
		}
		let mut collision = CollisionLayer::new(width, height);
		for x in 0..width {
			collision.set_collision_top(x,0);
			collision.set_collision_bottom(x, height);
		}
		for y in 0..height {
			collision.set_collision_left(0,y);
			collision.set_collision_right(width, y);
		}
		Building {
			position: Vector2u::new(x,y),
			tiles: Vec2D::from_vec(width, height, tiles),
			collision: collision,
		}
	}
}