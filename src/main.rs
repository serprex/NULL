#![allow(non_snake_case)]

extern crate num_bigint;
extern crate num_traits;
extern crate num_integer;
extern crate primal_sieve;
mod primes;

use std::collections::VecDeque;
use std::env;
use std::io::{self, Read, Write};
use std::fs;
use std::mem;
use num_bigint::BigUint;
use num_traits::{One, Zero, ToPrimitive, CheckedSub};
use primes::PrimeBag;

fn main() {
	let mut x = if let Some(a) = env::args().nth(1) {
		let mut f = fs::File::open(&a).expect("Failed to open file");
		let mut num = Vec::new();
		f.read_to_end(&mut num).expect("Failed to read number");
		num.retain(|&x| x >= b'0' && x <= b'9');
		BigUint::parse_bytes(&num, 10).unwrap()
	} else {
		println!("NULL [filename]");
		return
	};
	let mut y = BigUint::one();
	let mut primes = PrimeBag::new();
	let mut pidx = 0;
	let mut qs = [VecDeque::new(), VecDeque::new(), VecDeque::new()];
	let mut qi = 0;
	let mut out = io::stdout();
	let sin = io::stdin();
	let mut sinb = sin.bytes();
	let n255 = BigUint::from(255u8);
	let n2 = BigUint::from(2u8);
	while x >= n2 {
		match primes.minprime(&mut pidx, &mut x, &mut y) {
			0 => qi = if qi == 2 { 0 } else { qi + 1 },
			1 => qi = if qi == 0 { 2 } else { qi - 1 },
			2 => { out.write(&[*qs[qi].front().unwrap_or(&0)]).ok(); },
			3 =>
				if let Some(Ok(b)) = sinb.next() {
					if let Some(bm) = qs[qi].back_mut() {
						*bm = b;
						continue
					}
					qs[qi].push_back(b);
				},
			4 =>
				if let Some(ys) = qs[qi].front() {
					let ysb = BigUint::from(*ys);
					y = y.checked_sub(&ysb).unwrap_or_else(BigUint::zero);
				},
			5 =>
				if let Some(ys) = qs[qi].front() {
					let ysb = BigUint::from(*ys);
					y = y + ysb;
				},
			6 => {
				let y255 = (&y & &n255).to_u8().unwrap();
				if let Some(ys) = qs[qi].front_mut() {
					*ys = ys.wrapping_add(y255);
					continue
				}
				qs[qi].push_back(y255);
			},
			7 => {
				let b = qs[qi].pop_front().unwrap_or(0);
				qs[if qi == 2 { 0 } else { qi + 1 }].push_back(b);
			},
			8 => {
				let b = qs[qi].pop_front().unwrap_or(0);
				qs[if qi == 0 { 2 } else { qi - 1 }].push_back(b);

			},
			9 => { qs[qi].pop_front(); },
			10 => {
				let y255 = (&y & &n255).to_u8().unwrap();
				qs[qi].push_back(y255);
			},
			11 => {
				if if let Some(x) = qs[qi].front() {
					*x == 0
				} else {
					true
				} && x >= n2 {
					primes.minprime(&mut pidx, &mut x, &mut y);
				}
			},
			12 => {
				mem::swap(&mut x, &mut y);
				pidx = 0;
			},
			13 => return,
			_ => unreachable!(),
		}
	}
}
