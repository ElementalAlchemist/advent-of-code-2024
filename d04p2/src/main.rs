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
	for (y, line) in char_grid.iter().enumerate().skip(1) {
		for (x, c) in line.iter().copied().enumerate().skip(1) {
			if c != 'A' {
				continue;
			}
			let Some(upper_left) = char_grid.get(y - 1).and_then(|line| line.get(x - 1)) else {
				continue;
			};
			let Some(upper_right) = char_grid.get(y - 1).and_then(|line| line.get(x + 1)) else {
				continue;
			};
			let Some(lower_left) = char_grid.get(y + 1).and_then(|line| line.get(x - 1)) else {
				continue;
			};
			let Some(lower_right) = char_grid.get(y + 1).and_then(|line| line.get(x + 1)) else {
				continue;
			};

			if matches!(
				(upper_left, upper_right, lower_left, lower_right),
				('M', 'M', 'S', 'S') | ('M', 'S', 'M', 'S') | ('S', 'M', 'S', 'M') | ('S', 'S', 'M', 'M')
			) {
				found_words += 1;
			}
		}
	}

	println!("{}", found_words);

	Ok(())
}
