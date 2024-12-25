use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	let (locks, keys, height) = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut locks: Vec<Vec<u32>> = Vec::new();
		let mut keys: Vec<Vec<u32>> = Vec::new();
		let mut height = 0;
		let mut is_lock = false;
		let mut current_heights = Vec::new();
		for (y, line) in input_string.lines().enumerate() {
			if line.is_empty() {
				let heights = std::mem::take(&mut current_heights);
				if is_lock {
					locks.push(heights);
				} else {
					keys.push(heights);
				}
				if height == 0 {
					height = y;
				}
				continue;
			}
			if current_heights.is_empty() {
				is_lock = line.chars().all(|c| c == '#');
			}
			for (x, c) in line.chars().enumerate() {
				while x >= current_heights.len() {
					current_heights.push(0);
				}
				if c == '#' {
					current_heights[x] += 1;
				}
			}
		}
		if is_lock {
			locks.push(current_heights);
		} else {
			keys.push(current_heights);
		}
		(locks, keys, height as u32)
	};

	let mut combos = 0;
	for lock in locks.iter() {
		for key in keys.iter() {
			if lock.len() != key.len() {
				continue;
			}
			if lock
				.iter()
				.zip(key.iter())
				.all(|(lock_height, key_height)| *lock_height + *key_height <= height)
			{
				combos += 1;
			}
		}
	}

	println!("{}", combos);

	Ok(())
}
