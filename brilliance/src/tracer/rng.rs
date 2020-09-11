use super::*;
extern crate rand;
extern crate rand_xoshiro;
use rand::Rng as RandRng;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;

pub struct TileRng {
	shifts: Image<(f32, f32)>, // TODO: no need to store this unless iteratively refining.
	rng: Xoshiro256StarStar,
}

impl TileRng {
	pub fn new((w, h): (u32, u32), n: u32) -> Self {
		let mut rng = rand::thread_rng();
		Self {
			shifts: Image::from_fn((w, h), |_, _| (rng.gen(), rng.gen())),
			rng: Self::xoshiro(n),
		}
	}

	fn xoshiro(n: u32) -> Xoshiro256StarStar {
		let mut rng = Xoshiro256StarStar::seed_from_u64(0);
		for _ in 0..n {
			rng.jump();
		}
		rng
	}

	pub fn for_pix(&mut self, pix: (u32, u32), iter: u32) -> Rng {
		Rng {
			pix_shift: self.shifts.at(pix),
			iter: iter,
			rng: &mut self.rng,
		}
	}
}

pub struct Rng<'a> {
	pix_shift: (f32, f32),
	iter: u32,
	rng: &'a mut Xoshiro256StarStar,
}

impl Rng<'_> {
	pub fn quasi_random2(&self) -> (f32, f32) {
		halton23_scrambled(self.iter, self.pix_shift)
	}

	pub fn random(&mut self) -> f32 {
		self.rng.gen()
	}
}
