use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	let (mut left_list, mut right_list) = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut left_list: Vec<u32> = Vec::new();
		let mut right_list: Vec<u32> = Vec::new();

		for line in input_string.lines() {
			if line.is_empty() {
				continue;
			}
			let (left, right) = line.split_once("   ").expect("Delimiter not present");
			left_list.push(left.parse()?);
			right_list.push(right.parse()?);
		}

		(left_list, right_list)
	};

	left_list.sort_unstable();
	right_list.sort_unstable();

	let mut distance: u32 = 0;
	for (left, right) in left_list.iter().zip(right_list.iter()) {
		distance += left.abs_diff(*right);
	}

	println!("{}", distance);

	Ok(())
}
