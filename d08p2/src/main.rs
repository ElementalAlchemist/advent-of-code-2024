use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

#[derive(Eq, Hash, PartialEq)]
struct Coordinate {
	x: usize,
	y: usize,
}

fn gcd(mut a: usize, mut b: usize) -> usize {
	while b > 0 {
		std::mem::swap(&mut a, &mut b);
		b %= a;
	}
	a
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

				let slope = gcd(x_diff.max(y_diff), x_diff.min(y_diff));

				let x_diff = x_diff / slope;
				let y_diff = y_diff / slope;

				let mut node_x = Some(coordinate_1.x);
				let mut node_y = Some(coordinate_1.y);
				// Starting at coordinate 1, away from 2
				while let (Some(x), Some(y)) = (node_x, node_y) {
					antinodes.insert(Coordinate { x, y });

					if coordinate_1.x < coordinate_2.x {
						node_x = x.checked_sub(x_diff);
					} else {
						let next_x = x + x_diff;
						node_x = if next_x >= width { None } else { Some(next_x) };
					}

					if coordinate_1.y < coordinate_2.y {
						node_y = y.checked_sub(y_diff);
					} else {
						let next_y = y + y_diff;
						node_y = if next_y >= height { None } else { Some(next_y) };
					}
				}
				// Starting at coordinate 1, through 2
				node_x = Some(coordinate_1.x);
				node_y = Some(coordinate_1.y);
				while let (Some(x), Some(y)) = (node_x, node_y) {
					antinodes.insert(Coordinate { x, y });

					if coordinate_1.x > coordinate_2.x {
						node_x = x.checked_sub(x_diff);
					} else {
						let next_x = x + x_diff;
						node_x = if next_x >= width { None } else { Some(next_x) };
					}
					if coordinate_1.y > coordinate_2.y {
						node_y = y.checked_sub(y_diff);
					} else {
						let next_y = y + y_diff;
						node_y = if next_y >= height { None } else { Some(next_y) };
					}
				}
			}
		}
	}

	println!("{}", antinodes.len());

	Ok(())
}
