use std::error::Error;
use std::fs;

const PRUNE_BASE: u64 = 16777216;

fn next_secret_number(mut secret_number: u64) -> u64 {
	let step_1 = secret_number * 64;
	secret_number ^= step_1;
	secret_number %= PRUNE_BASE;
	let step_2 = secret_number / 32;
	secret_number ^= step_2;
	secret_number %= PRUNE_BASE;
	let step_3 = secret_number * 2048;
	secret_number ^= step_3;
	secret_number %= PRUNE_BASE;
	secret_number
}

fn main() -> Result<(), Box<dyn Error>> {
	let seller_secret_numbers = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut secret_numbers: Vec<u64> = Vec::new();
		for line in input_string.lines() {
			secret_numbers.push(line.parse()?);
		}
		secret_numbers
	};

	let mut final_secret_numbers = Vec::new();
	for mut secret_number in seller_secret_numbers {
		for _ in 0..2000 {
			secret_number = next_secret_number(secret_number);
		}
		final_secret_numbers.push(secret_number);
	}

	let result: u64 = final_secret_numbers.into_iter().sum();
	println!("{}", result);

	Ok(())
}
