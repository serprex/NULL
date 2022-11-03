#![allow(non_snake_case)]

mod primes;
mod vm;

use std::env;
use std::fs;
use std::io::{self, Read};
use num_bigint::BigUint;

fn main() {
	let Some(a) = env::args().nth(1) else {
		println!("NULL [filename]");
		return;
	};
	let mut f = fs::File::open(&a).expect("Failed to open file");
	let mut num = Vec::new();
	f.read_to_end(&mut num).expect("Failed to read number");
	num.retain(|&x| x >= b'0' && x <= b'9');
	vm::run(BigUint::parse_bytes(&num, 10).unwrap(), io::stdin(), io::stdout());
}
