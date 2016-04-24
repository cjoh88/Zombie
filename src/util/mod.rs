use std::iter::Iterator;
use std::ops::{Deref, DerefMut, Index, IndexMut};

const tile_size: u32 = 64;

pub fn get_tile_texture_coords(i: u32) -> (f32, f32, f32, f32) {
	let x = ((i * tile_size) % 640) as f32;
	let y = ((i * tile_size) / 640) as f32;
	(x,y,x+1f32, y+1f32)
}

pub fn itoc(i: usize, width: usize) -> (usize, usize) {
	(i % width, i / width)
}

pub fn ctoi(x: usize, y: usize, width: usize) -> usize {
	y * width + x
}

pub struct Vec2D<T> {
	width: usize,
	height: usize,
	vec: Vec<T>,
}

impl<T> Vec2D<T> {
	pub fn new(width: usize, height: usize) -> Self {
		Vec2D{
			width: width,
			height: height,
			vec: Vec::with_capacity(width * height)
		}
	}

	pub fn len(&self) -> usize {
		self.vec.len()
	}

	pub fn width(&self) -> usize {
		self.width
	}

	pub fn height(&self) -> usize {
		self.height
	}

	pub fn get(&self, x: usize, y:usize) -> Option<&T> {
		self.vec.get(y * self.width + x)
	}

	pub fn get_mut(&mut self, x: usize, y:usize) -> Option<&mut T> {
		self.vec.get_mut(y * self.width + x)
	}
}

impl<T> Deref for Vec2D<T> {
	type Target = Vec<T>;

	fn deref(&self) -> &Vec<T> {
		&self.vec
	}
}

impl<T> DerefMut for Vec2D<T> {
	fn deref_mut(&mut self) -> &mut Vec<T> {
		&mut self.vec
	}
}

impl<T> Index<(usize, usize)> for Vec2D<T> {
	type Output = T;

	fn index(&self, (x,y): (usize, usize)) -> &T {
		&self.vec[y * self.width + x]
	}
}

impl<T> IndexMut<(usize, usize)> for Vec2D<T> {
	fn index_mut(&mut self, (x,y): (usize, usize)) -> &mut T {
		&mut self.vec[y * self.width + x]
	}
}