use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

const TIME_SAVE_THRESHOLD: usize = 100;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
	x: usize,
	y: usize,
}

impl Coordinate {
	fn adjacent(&self, width: usize, height: usize) -> Vec<Self> {
		let mut adjacent = Vec::new();
		if self.x > 0 {
			adjacent.push(Self {
				x: self.x - 1,
				y: self.y,
			});
		}
		if self.y > 0 {
			adjacent.push(Self {
				x: self.x,
				y: self.y - 1,
			});
		}
		if self.x < width - 1 {
			adjacent.push(Self {
				x: self.x + 1,
				y: self.y,
			});
		}
		if self.y < height - 1 {
			adjacent.push(Self {
				x: self.x,
				y: self.y + 1,
			});
		}
		adjacent
	}

	fn twenty_adjacent(&self, width: usize, height: usize) -> HashMap<Self, usize> {
		let mut current = vec![self.clone()];
		let mut found: HashMap<Self, usize> = HashMap::new();
		found.insert(self.clone(), 0);
		for dist in 1..=20 {
			let mut next = Vec::new();
			for coord in current {
				for next_coord in coord.adjacent(width, height) {
					found.entry(next_coord.clone()).or_insert_with(|| {
						next.push(next_coord);
						dist
					});
				}
			}
			current = next;
		}
		found.remove(self);
		found
	}
}

#[derive(Clone)]
struct PathProgress {
	current: Coordinate,
	path: Vec<Coordinate>,
}

impl PathProgress {
	fn new(start: Coordinate) -> Self {
		let path = vec![start.clone()];
		let current = start;
		Self { current, path }
	}

	fn set_current(&mut self, new_current: Coordinate) {
		self.path.push(new_current.clone());
		self.current = new_current;
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let (walls, start, end, width, height) = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut walls: HashSet<Coordinate> = HashSet::new();
		let mut start: Option<Coordinate> = None;
		let mut end: Option<Coordinate> = None;
		let mut width = 0;
		let mut height = 0;
		for (y, line) in input_string.lines().enumerate() {
			height += 1;
			width = line.len();
			for (x, c) in line.chars().enumerate() {
				match c {
					'S' => start = Some(Coordinate { x, y }),
					'E' => end = Some(Coordinate { x, y }),
					'#' => {
						walls.insert(Coordinate { x, y });
					}
					_ => (),
				}
			}
		}

		let start = start.expect("Starting coordinate");
		let end = end.expect("Ending coordinate");

		(walls, start, end, width, height)
	};

	let mut current_paths = vec![PathProgress::new(start.clone())];
	let mut visited: HashSet<Coordinate> = HashSet::new();
	let mut final_path: Option<PathProgress> = None;
	'pathfind: while !current_paths.is_empty() {
		let mut new_paths = Vec::new();
		for path in current_paths {
			for adjacent in path.current.adjacent(width, height) {
				if walls.contains(&adjacent) || visited.contains(&adjacent) {
					continue;
				}
				visited.insert(adjacent.clone());
				let mut new_path = path.clone();
				new_path.set_current(adjacent);
				if new_path.current == end {
					final_path = Some(new_path);
					break 'pathfind;
				}
				new_paths.push(new_path);
			}
		}
		current_paths = new_paths;
	}

	let Some(final_path) = final_path else {
		println!("No path found.");
		return Ok(());
	};

	let path_visited: HashMap<Coordinate, usize> = final_path
		.path
		.iter()
		.cloned()
		.enumerate()
		.map(|(index, coord)| (coord, index))
		.collect();
	let mut saves_enough_time = 0;
	for (index, coord) in final_path.path.iter().enumerate() {
		for (next_coord, dist) in coord.twenty_adjacent(width, height) {
			if let Some(next_index) = path_visited.get(&next_coord) {
				if *next_index > index && *next_index - index - dist >= TIME_SAVE_THRESHOLD {
					saves_enough_time += 1;
				}
			}
		}
	}

	println!("{}", saves_enough_time);

	Ok(())
}
