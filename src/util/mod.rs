//use std::iter::Iterator;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::slice::{Iter, IterMut};

const TILE_SIZE: u32 = 64;

pub fn get_tile_texture_coords(i: u32) -> (f32, f32, f32, f32) {
	let x = ((i * TILE_SIZE) % 640) as f32;
	let y = ((i * TILE_SIZE) / 640) as f32;
	(x,y,x+1f32, y+1f32)
}

#[allow(dead_code)]
pub fn itoc(i: usize, width: usize) -> (usize, usize) {
	(i % width, i / width)
}

#[allow(dead_code)]
pub fn ctoi(x: usize, y: usize, width: usize) -> usize {
	y * width + x
}

#[allow(dead_code)]
pub struct Vec2D<T> {
	width: usize,
	height: usize,
	vec: Vec<T>,
}

#[allow(dead_code)]
impl<T> Vec2D<T> {
	pub fn new(width: usize, height: usize) -> Self {
		Vec2D{
			width: width,
			height: height,
			vec: Vec::with_capacity(width * height)
		}
	}

	pub fn from_vec(width: usize, height: usize, vec: Vec<T>) -> Self {
		Vec2D {
			width: width,
			height: height,
			vec: vec,
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

	pub fn iter(&self) -> Iter<T> {
		self.vec.iter()
	}

	pub fn iter_mut(&mut self) -> IterMut<T> {
		self.vec.iter_mut()
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

/*
pub struct NeighborIterator<'a, T> {
	vec_2d: &'a Vec2D<T>,
	x: usize,
	y: usize,
}



impl<'a, T> Iterator for NeighborIterator<'a, T> {
	type Item = &'a T;

	fn next(&mut self) -> Option<&'a T> {
		unimplemented!();
	}
}*/