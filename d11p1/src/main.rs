use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	let mut stones = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut stones: Vec<u64> = Vec::new();
		for stone in input_string.split(' ') {
			stones.push(stone.parse()?);
		}

		stones
	};

	for _ in 0..25 {
		let mut new_stones = Vec::new();
		for stone in stones {
			if stone == 0 {
				new_stones.push(1);
			} else {
				let stone_number = stone.to_string();
				let stone_len = stone_number.len();
				if stone_len % 2 == 0 {
					let (left_stone, right_stone) = stone_number.split_at(stone_len / 2);
					new_stones.push(left_stone.parse()?);
					new_stones.push(right_stone.parse()?);
				} else {
					new_stones.push(stone * 2024);
				}
			}
		}
		stones = new_stones;
	}

	println!("{}", stones.len());

	Ok(())
}
