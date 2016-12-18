use num::{BigUint, Integer, Zero};
use primal_sieve::Primes;

pub struct PrimeBag {
	prev: Vec<BigUint>,
	pit: Primes,
}

impl PrimeBag {
	pub fn new() -> PrimeBag {
		PrimeBag {
			prev: Vec::new(),
			pit: Primes::all(),
		}
	}

	pub fn minprime(&mut self, x: &mut BigUint, y: &mut BigUint) -> usize {
		let mut idx = 0;
		loop {
			let len = self.prev.len();
			if idx == len {
				let p = BigUint::from(self.pit.next().unwrap());
				self.prev.push(p);
			}
			let p = &self.prev[idx];
			let (div, rem) = x.div_rem(p);
			if rem.is_zero() {
				*x = div;
				*y = &*y * p;
				return idx % 14
			}
			idx += 1;
		}
	}
}
