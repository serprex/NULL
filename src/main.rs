#![allow(non_snake_case)]

extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;
extern crate primal_sieve;
mod primes;
mod vm;

use std::env;
use std::fs;
use std::io::{self, Read};
use num_bigint::BigUint;

fn main() {
	let x = if let Some(a) = env::args().nth(1) {
		let mut f = fs::File::open(&a).expect("Failed to open file");
		let mut num = Vec::new();
		f.read_to_end(&mut num).expect("Failed to read number");
		num.retain(|&x| x >= b'0' && x <= b'9');
		BigUint::parse_bytes(&num, 10).unwrap()
	} else {
		println!("NULL [filename]");
		return;
	};

	vm::run(x, io::stdin(), io::stdout());
}
