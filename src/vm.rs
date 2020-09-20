use std::collections::VecDeque;
use std::io::{Read, Write};
use std::mem;
use num_bigint::BigUint;
use num_traits::{CheckedSub, One, ToPrimitive, Zero};
use primes::PrimeBag;

pub fn run<R, W>(mut x: BigUint, input: R, mut out: W)
	where R: Read, W: Write
{
	let mut y = BigUint::one();
	let mut primes = PrimeBag::new();
	let mut pidx = 0;
	let mut qs = (VecDeque::new(), VecDeque::new(), VecDeque::new());
	let mut sinb = input.bytes();
	let n255 = BigUint::from(255u8);
	let n2 = BigUint::from(2u8);
	while x >= n2 {
		match primes.minprime(&mut pidx, &mut x, &mut y) {
			0 => qs = (qs.1, qs.2, qs.0),
			1 => qs = (qs.2, qs.0, qs.1),
			2 => {
				out.write(&[*qs.0.front().unwrap_or(&0)]).ok();
			}
			3 => if let Some(Ok(b)) = sinb.next() {
				if let Some(bm) = qs.0.back_mut() {
					*bm = b;
					continue;
				}
				qs.0.push_back(b);
			},
			4 => if let Some(ys) = qs.0.front() {
				let ysb = BigUint::from(*ys);
				y = y.checked_sub(&ysb).unwrap_or_else(BigUint::zero);
			},
			5 => if let Some(ys) = qs.0.front() {
				let ysb = BigUint::from(*ys);
				y = y + ysb;
			},
			6 => {
				let y255 = (&y & &n255).to_u8().unwrap();
				if let Some(ys) = qs.0.front_mut() {
					*ys = ys.wrapping_add(y255);
					continue;
				}
				qs.0.push_back(y255);
			}
			7 => {
				let b = qs.0.pop_front().unwrap_or(0);
				qs.1.push_back(b);
			}
			8 => {
				let b = qs.0.pop_front().unwrap_or(0);
				qs.2.push_back(b);
			}
			9 => {
				qs.0.pop_front();
			}
			10 => {
				let y255 = (&y & &n255).to_u8().unwrap();
				qs.0.push_back(y255);
			}
			11 => {
				if if let Some(x) = qs.0.front() {
					*x == 0
				} else {
					true
				} && x >= n2
				{
					primes.minprime(&mut pidx, &mut x, &mut y);
				}
			}
			12 => {
				mem::swap(&mut x, &mut y);
				pidx = 0;
			}
			13 => return,
			_ => unreachable!(),
		}
	}
}

#[cfg(test)]
mod tests {
	use num_bigint::BigUint;

	#[test]
	fn hello_world() {
		let input = b"";
		let mut output = Vec::<u8>::new();
		super::run(
			BigUint::parse_bytes(b"18090462148251759497492444420325028573004825667450262208483921113691874262881209112703483826587581124351159753006294894679414849393349134822194686265244710288508550347259", 10).unwrap(),
			&input[..],
			&mut output
		);
		assert_eq!(&output[..], b"Hello, World!\n");
	}
}
