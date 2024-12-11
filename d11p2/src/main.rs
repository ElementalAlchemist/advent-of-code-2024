use std::collections::HashMap;
use std::error::Error;
use std::fs;

const BLINK_COUNT: u32 = 75;

fn main() -> Result<(), Box<dyn Error>> {
	let mut stones = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut stones: HashMap<u64, u64> = HashMap::new();
		for stone in input_string.split(' ') {
			let stone: u64 = stone.parse()?;
			*stones.entry(stone).or_default() += 1;
		}

		stones
	};

	for _ in 0..BLINK_COUNT {
		let mut new_stones: HashMap<u64, u64> = HashMap::new();
		for (stone, count) in stones {
			if stone == 0 {
				*new_stones.entry(1).or_default() += count;
			} else {
				let stone_digits = stone.ilog10() + 1;
				if stone_digits % 2 == 0 {
					let split = 10u64.pow(stone_digits / 2);
					let left_stone = stone / split;
					let right_stone = stone % split;
					*new_stones.entry(left_stone).or_default() += count;
					*new_stones.entry(right_stone).or_default() += count;
				} else {
					*new_stones.entry(stone * 2024).or_default() += count;
				}
			}
		}
		stones = new_stones;
	}

	let final_stones: u64 = stones.values().sum();
	println!("{}", final_stones);

	Ok(())
}
