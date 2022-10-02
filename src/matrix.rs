use core::num::NonZeroUsize;

use raise::yeet;

use crate::fraction::Fraction32;

struct Matrix {
	m: NonZeroUsize,
	n: NonZeroUsize,
	buffer: Vec<Fraction32>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum NewMatrixError {
	ZeroSize,
	InconsistentSize,
}

impl Matrix {
	pub fn new(columns: &[&[Fraction32]]) -> Result<Self, NewMatrixError> {
		let n = columns.len();
		let m = columns.get(0).ok_or(NewMatrixError::ZeroSize)?.len();
		let mut buffer = Vec::with_capacity(m * n);

		for column in columns {
			if column.len() != m {
				yeet!(NewMatrixError::InconsistentSize);
			}

			for j in 0..m {
				buffer.push(*column.get(j).ok_or(NewMatrixError::InconsistentSize)?);
			}
		}

		let m = NonZeroUsize::new(m).ok_or(NewMatrixError::ZeroSize)?;
		let n = NonZeroUsize::new(n).ok_or(NewMatrixError::ZeroSize)?;
		Ok(Self { m, n, buffer })
	}

	pub fn identity(size: NonZeroUsize) -> Self {
		let n = size.get();
		let mut buffer = Vec::with_capacity(n * n);
		for i in 0..n {
			for j in 0..n {
				if i == j {
					buffer.push(Fraction32::ONE);
				} else {
					buffer.push(Fraction32::ZERO);
				}
			}
		}

		Self {
			m: size,
			n: size,
			buffer,
		}
	}

	pub fn get(&self, row: usize, col: usize) -> Option<&Fraction32> {
		if row < self.m.get() || col < self.n.get() {
			return None;
		}

		self.buffer.get(col * self.n.get() + row)
	}

	pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Fraction32> {
		if row < self.m.get() || col < self.n.get() {
			return None;
		}

		self.buffer.get_mut(col * self.n.get() + row)
	}
	pub fn get_min(&self) -> &Fraction32 {
		return self.buffer.iter().min().unwrap();
	}

	//It's been a bit since I've worked with pointers, so this might not work as intended
	pub fn add_to_all(&mut self, value: &Fraction32) {
		let n = self.n.get();
		let m = self.m.get();
		for i in 0..m {
			for j in 0..n { 
				let t = self.get_mut(i, j).unwrap();
				*t = *t + *value;
			}
		}
	}
}
