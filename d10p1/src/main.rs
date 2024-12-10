use std::collections::HashSet;
use std::error::Error;
use std::fs;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
	x: usize,
	y: usize,
}

impl Coordinate {
	fn next_in_all_directions(&self) -> Vec<Coordinate> {
		let mut next = vec![
			Coordinate {
				x: self.x + 1,
				y: self.y,
			},
			Coordinate {
				x: self.x,
				y: self.y + 1,
			},
		];
		if self.x > 0 {
			next.push(Coordinate {
				x: self.x - 1,
				y: self.y,
			});
		}
		if self.y > 0 {
			next.push(Coordinate {
				x: self.x,
				y: self.y - 1,
			});
		}
		next
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let land_heights: Vec<Vec<u32>> = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut heights = Vec::new();
		for line in input_string.lines() {
			let mut row_heights = Vec::new();
			for c in line.chars() {
				row_heights.push((c as u32) - 48);
			}
			heights.push(row_heights);
		}

		heights
	};

	let mut trailheads: Vec<Coordinate> = Vec::new();
	for (y, row) in land_heights.iter().enumerate() {
		for (x, cell) in row.iter().enumerate() {
			if *cell == 0 {
				trailheads.push(Coordinate { x, y });
			}
		}
	}

	let mut all_trailheads_scores: u32 = 0;
	for trailhead in trailheads {
		let mut pointers: HashSet<Coordinate> = HashSet::new();
		pointers.insert(trailhead);
		while !pointers.is_empty() {
			let mut new_pointers: HashSet<Coordinate> = HashSet::new();
			for pointer in pointers.iter() {
				let current_number = land_heights[pointer.y][pointer.x];
				if current_number == 9 {
					all_trailheads_scores += 1;
					continue;
				}
				let next_number = current_number + 1;
				for next_pointer in pointer.next_in_all_directions() {
					if land_heights
						.get(next_pointer.y)
						.and_then(|row| row.get(next_pointer.x))
						.copied() == Some(next_number)
					{
						new_pointers.insert(next_pointer);
					}
				}
			}
			pointers = new_pointers;
		}
	}

	println!("{}", all_trailheads_scores);

	Ok(())
}
