use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

#[derive(Eq, Hash, PartialEq)]
struct Coordinate {
	x: usize,
	y: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
	let (antennae_by_frequency, width, height) = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut antennae_by_frequency: HashMap<char, Vec<Coordinate>> = HashMap::new();
		let mut width = 0;
		let mut height = 0;
		for (y, line) in input_string.lines().enumerate() {
			height += 1;
			width = line.len();
			for (x, c) in line.chars().enumerate() {
				if c.is_ascii_alphanumeric() {
					let coordinate = Coordinate { x, y };
					antennae_by_frequency.entry(c).or_default().push(coordinate);
				}
			}
		}

		(antennae_by_frequency, width, height)
	};

	let mut antinodes: HashSet<Coordinate> = HashSet::new();
	for antenna_coordinates in antennae_by_frequency.values() {
		for (index, coordinate_1) in antenna_coordinates.iter().enumerate() {
			for coordinate_2 in antenna_coordinates.iter().skip(index + 1) {
				let x_diff = coordinate_1.x.abs_diff(coordinate_2.x);
				let y_diff = coordinate_1.y.abs_diff(coordinate_2.y);

				let antinode_0_x = if coordinate_1.x < coordinate_2.x {
					coordinate_1.x.checked_sub(x_diff)
				} else {
					let x = coordinate_1.x + x_diff;
					if x >= width {
						None
					} else {
						Some(x)
					}
				};
				let antinode_0_y = if coordinate_1.y < coordinate_2.y {
					coordinate_1.y.checked_sub(y_diff)
				} else {
					let y = coordinate_1.y + y_diff;
					if y >= height {
						None
					} else {
						Some(y)
					}
				};

				let antinode_3_x = if coordinate_2.x < coordinate_1.x {
					coordinate_2.x.checked_sub(x_diff)
				} else {
					let x = coordinate_2.x + x_diff;
					if x >= width {
						None
					} else {
						Some(x)
					}
				};
				let antinode_3_y = if coordinate_2.y < coordinate_1.y {
					coordinate_2.y.checked_sub(y_diff)
				} else {
					let y = coordinate_2.y + y_diff;
					if y >= height {
						None
					} else {
						Some(y)
					}
				};

				if let (Some(x), Some(y)) = (antinode_0_x, antinode_0_y) {
					antinodes.insert(Coordinate { x, y });
				}
				if let (Some(x), Some(y)) = (antinode_3_x, antinode_3_y) {
					antinodes.insert(Coordinate { x, y });
				}
			}
		}
	}

	println!("{}", antinodes.len());

	Ok(())
}
