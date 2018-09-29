use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::Zero;
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

	pub fn minprime(&mut self, idx: &mut usize, x: &mut BigUint, y: &mut BigUint) -> usize {
		loop {
			let len = self.prev.len();
			if *idx == len {
				let p = BigUint::from(self.pit.next().unwrap());
				self.prev.push(p);
			}
			let p = &self.prev[*idx];
			let (div, rem) = x.div_rem(p);
			if rem.is_zero() {
				*x = div;
				*y = &*y * p;
				return *idx % 14;
			}
			*idx += 1;
		}
	}
}
