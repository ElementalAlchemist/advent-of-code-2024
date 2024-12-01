use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	let (left_list, right_list) = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut left_list: Vec<u32> = Vec::new();
		let mut right_list: HashMap<u32, u32> = HashMap::new();

		for line in input_string.lines() {
			if line.is_empty() {
				continue;
			}
			let (left, right) = line.split_once("   ").expect("Delimiter not present");
			left_list.push(left.parse()?);
			let right_number: u32 = right.parse()?;
			*right_list.entry(right_number).or_default() += 1;
		}

		(left_list, right_list)
	};

	let mut score: u32 = 0;
	for left in left_list.iter() {
		score += *left * *right_list.get(left).unwrap_or(&0);
	}

	println!("{}", score);

	Ok(())
}
