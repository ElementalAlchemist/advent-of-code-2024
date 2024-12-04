use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	let char_grid = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut char_grid: Vec<Vec<char>> = Vec::new();
		for line in input_string.lines() {
			let char_line: Vec<char> = line.chars().collect();
			char_grid.push(char_line);
		}

		char_grid
	};

	let mut found_words: u32 = 0;
	for (y_coord, line) in char_grid.iter().enumerate() {
		let y = y_coord as i64;
		for (x_coord, c) in line.iter().copied().enumerate() {
			let x = x_coord as i64;
			if c != 'X' {
				continue;
			}
			for x_shift in -1..=1 {
				if x_shift < 0 && x < 3 {
					continue;
				}
				for y_shift in -1..=1 {
					if y_shift < 0 && y < 3 {
						continue;
					}
					if x_shift == 0 && y_shift == 0 {
						continue;
					}
					if char_grid
						.get((y + y_shift) as usize)
						.map(|line| line.get((x + x_shift) as usize))
						.map(|c| c == Some(&'M'))
						.unwrap_or(false) && char_grid
						.get((y + y_shift * 2) as usize)
						.map(|line| line.get((x + x_shift * 2) as usize))
						.map(|c| c == Some(&'A'))
						.unwrap_or(false) && char_grid
						.get((y + y_shift * 3) as usize)
						.map(|line| line.get((x + x_shift * 3) as usize))
						.map(|c| c == Some(&'S'))
						.unwrap_or(false)
					{
						found_words += 1;
					}
				}
			}
		}
	}

	println!("{}", found_words);

	Ok(())
}
