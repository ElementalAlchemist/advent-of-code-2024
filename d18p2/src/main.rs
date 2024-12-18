use std::collections::HashSet;
use std::error::Error;
use std::fs;

const MAX_WIDTH: u32 = 70;
const MAX_HEIGHT: u32 = 70;
const START_CORRUPTED: usize = 1024;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
	x: u32,
	y: u32,
}

impl Coordinate {
	fn all_directions(&self) -> Vec<Coordinate> {
		let mut directions = Vec::new();
		if self.x > 0 {
			directions.push(Coordinate {
				x: self.x - 1,
				y: self.y,
			});
		}
		if self.y > 0 {
			directions.push(Coordinate {
				x: self.x,
				y: self.y - 1,
			});
		}
		if self.x < MAX_WIDTH {
			directions.push(Coordinate {
				x: self.x + 1,
				y: self.y,
			});
		}
		if self.y < MAX_HEIGHT {
			directions.push(Coordinate {
				x: self.x,
				y: self.y + 1,
			});
		}
		directions
	}
}

fn has_path(corrupted_coords: &HashSet<Coordinate>) -> bool {
	let start = Coordinate { x: 0, y: 0 };
	let mut current_coordinates = vec![start.clone()];
	let mut visited = HashSet::new();
	visited.insert(start);

	let destination = Coordinate {
		x: MAX_WIDTH,
		y: MAX_HEIGHT,
	};
	while !current_coordinates.is_empty() {
		let mut new_coords = Vec::new();
		for coord in current_coordinates {
			for next_coord in coord.all_directions() {
				if next_coord == destination {
					return true;
				}
				if corrupted_coords.contains(&next_coord) || visited.contains(&next_coord) {
					continue;
				}
				new_coords.push(next_coord.clone());
				visited.insert(next_coord);
			}
		}
		current_coordinates = new_coords;
	}

	false
}

fn main() -> Result<(), Box<dyn Error>> {
	let byte_coordinates = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut coordinates: Vec<Coordinate> = Vec::new();
		for line in input_string.lines() {
			let (x, y) = line.split_once(',').expect("Coordinate");
			let x = x.parse()?;
			let y = y.parse()?;
			coordinates.push(Coordinate { x, y });
		}

		coordinates
	};

	let mut corrupted: HashSet<Coordinate> = HashSet::new();
	let mut bytes_iter = byte_coordinates.into_iter();
	for _ in 0..START_CORRUPTED {
		corrupted.insert(bytes_iter.next().unwrap());
	}

	for byte in bytes_iter {
		corrupted.insert(byte.clone());
		if !has_path(&corrupted) {
			println!("{},{}", byte.x, byte.y);
			break;
		}
	}

	Ok(())
}
