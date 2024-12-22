use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fs;

const PRUNE_BASE: i64 = 16777216;

fn next_secret_number(mut secret_number: i64) -> i64 {
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

		let mut secret_numbers: Vec<i64> = Vec::new();
		for line in input_string.lines() {
			secret_numbers.push(line.parse()?);
		}
		secret_numbers
	};

	let mut sequence_tracking = Vec::new();
	for secret_number in seller_secret_numbers {
		let mut previous_number = secret_number;
		let mut changes = VecDeque::new();
		let mut sequence_to_price_map: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
		for _ in 0..2000 {
			let next_number = next_secret_number(previous_number);
			let price = next_number % 10;
			let change = price - (previous_number % 10);
			changes.push_back(change);
			previous_number = next_number;
			if changes.len() < 4 {
				continue;
			}
			while changes.len() > 4 {
				changes.pop_front();
			}
			let sequence = (changes[0], changes[1], changes[2], changes[3]);
			if let Entry::Vacant(sequence_entry) = sequence_to_price_map.entry(sequence) {
				sequence_entry.insert(price);
			}
		}
		sequence_tracking.push(sequence_to_price_map);
	}

	let mut most_bananas = 0;
	let mut most_bananas_sequence = (0, 0, 0, 0);
	for first in -18..=18 {
		for second in -18..=18 {
			for third in -18..=18 {
				for fourth in -18..=18 {
					let sequence = (first, second, third, fourth);
					let mut earned_bananas = 0;
					for seller in sequence_tracking.iter() {
						if let Some(price) = seller.get(&sequence) {
							earned_bananas += *price;
						}
					}
					if earned_bananas > most_bananas {
						most_bananas = earned_bananas;
						most_bananas_sequence = sequence;
					}
				}
			}
		}
	}

	println!("{:?}", most_bananas_sequence);
	println!("{}", most_bananas);

	Ok(())
}
