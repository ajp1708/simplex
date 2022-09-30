use core::num::NonZeroU16;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Uses Stein's algorithm to calculate the gcd of two numbers
const fn gcd(mut a: u16, mut b: u16) -> u16 {
	// returns the other if one of the two numbers are zero
	if a == 0 || b == 0 {
		return a | b;
	}

	// find common factors of two
	let shift = (a | b).trailing_zeros();

	// divide both by two until they're odd
	a >>= a.trailing_zeros();
	b >>= b.trailing_zeros();

	while a != b {
		if a > b {
			a -= b;
			a >>= a.trailing_zeros();
		} else {
			b -= a;
			b >>= b.trailing_zeros();
		}
	}

	a << shift
}

const fn lcm(a: u16, b: u16) -> u16 {
	let gcd = gcd(a, b);
	a * b / gcd
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fraction32 {
	numerator: i16,
	denominator: NonZeroU16,
}

impl Fraction32 {
	pub const ZERO: Self = Self::whole(0);
	pub const ONE: Self = Self::whole(1);
	pub const NEG_ONE: Self = Self::whole(-1);

	/// Create a new fraction
	///
	/// # Panics
	///
	/// This panics if the denominator is larger than `i16::MAX`
	#[must_use]
	pub fn new(numerator: i16, denominator: NonZeroU16) -> Self {
		let this = Self {
			numerator,
			denominator,
		};

		// check for a denominator that's too large
		assert!(denominator.get() <= i16::MAX.unsigned_abs());

		// simplify the fraction
		this.reduce()
	}

	/// Create a fraction from a whole number
	#[must_use]
	pub const fn whole(num: i16) -> Self {
		// safety: one is neither zero, nor greater than 35,000
		unsafe { Self::new_unchecked(num, 1) }
	}

	/// Create a new fraction
	///
	/// # Safety
	///
	/// The `denominator` cannot be zero, or larger than `i16::MAX`
	#[must_use]
	pub const unsafe fn new_unchecked(numerator: i16, denominator: u16) -> Self {
		Self {
			numerator,
			denominator: NonZeroU16::new_unchecked(denominator),
		}
	}

	#[must_use]
	pub const fn numerator(self) -> i16 {
		self.numerator
	}

	#[must_use]
	pub const fn denominator(self) -> NonZeroU16 {
		self.denominator
	}

	/// Simplify the fraction
	#[must_use]
	#[allow(clippy::missing_panics_doc)]
	pub fn reduce(self) -> Self {
		if self.numerator == 0 {
			return Self::ZERO;
		}

		let gcd = gcd(self.numerator.unsigned_abs(), self.denominator.get());
		let numerator = self.numerator / i16::try_from(gcd).unwrap();
		let denominator = self.denominator.get() / gcd;

		Self::new(numerator, denominator.try_into().unwrap())
	}

	/// Returns the reciprocal of the fraction.
	/// Returns `None` if the numerator is currently zero.
	#[must_use]
	#[allow(clippy::missing_panics_doc)]
	pub fn reciprocal(self) -> Option<Self> {
		let numerator = i16::try_from(self.denominator.get()).unwrap() * self.numerator.signum();
		let denominator = self.numerator.unsigned_abs().try_into().ok()?;

		Some(Self::new(numerator, denominator))
	}
}

impl Add<Self> for Fraction32 {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		let denominator = lcm(self.denominator.get(), rhs.denominator.get());
		let numerator = self.numerator + rhs.numerator;
		Self::new(numerator, NonZeroU16::new(denominator).unwrap())
	}
}

impl AddAssign<Self> for Fraction32 {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
	}
}

impl Sub<Self> for Fraction32 {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		self.add(rhs.mul(Self::NEG_ONE))
	}
}

impl SubAssign<Self> for Fraction32 {
	fn sub_assign(&mut self, rhs: Self) {
		*self = *self - rhs;
	}
}

impl Mul<Self> for Fraction32 {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		let numerator = self.numerator * rhs.numerator;
		let denominator = self.denominator.checked_mul(rhs.denominator).unwrap();

		Self::new(numerator, denominator)
	}
}

impl MulAssign<Self> for Fraction32 {
	fn mul_assign(&mut self, rhs: Self) {
		*self = *self * rhs;
	}
}

impl Div<Self> for Fraction32 {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		self.mul(rhs.reciprocal().unwrap())
	}
}

impl DivAssign<Self> for Fraction32 {
	fn div_assign(&mut self, rhs: Self) {
		*self = *self / rhs;
	}
}
