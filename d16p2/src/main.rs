use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::error::Error;
use std::fs;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Coordinate {
	x: usize,
	y: usize,
}

impl Coordinate {
	fn advance_in_direction(&self, direction: Direction) -> Option<Self> {
		match direction {
			Direction::North => {
				if self.y > 0 {
					Some(Self {
						x: self.x,
						y: self.y - 1,
					})
				} else {
					None
				}
			}
			Direction::East => Some(Self {
				x: self.x + 1,
				y: self.y,
			}),
			Direction::South => Some(Self {
				x: self.x,
				y: self.y + 1,
			}),
			Direction::West => {
				if self.x > 0 {
					Some(Self {
						x: self.x - 1,
						y: self.y,
					})
				} else {
					None
				}
			}
		}
	}
}

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Direction {
	North,
	East,
	South,
	West,
}

impl Direction {
	fn get_adjacent(&self) -> Vec<Direction> {
		match self {
			Self::North => vec![Self::East, Self::West],
			Self::East => vec![Self::South, Self::North],
			Self::South => vec![Self::West, Self::East],
			Self::West => vec![Self::North, Self::South],
		}
	}
}

impl Default for Direction {
	fn default() -> Self {
		Self::East
	}
}

#[derive(Eq, PartialEq)]
struct TravelState {
	score_so_far: u32,
	facing_direction: Direction,
	current_position: Coordinate,
	path_tiles: Vec<Coordinate>,
}

impl Ord for TravelState {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.score_so_far
			.cmp(&other.score_so_far)
			.then_with(|| self.current_position.cmp(&other.current_position))
			.then_with(|| self.facing_direction.cmp(&other.facing_direction))
			.then_with(|| self.path_tiles.cmp(&other.path_tiles))
	}
}

impl PartialOrd for TravelState {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let (start, end, walls) = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut start = Coordinate { x: 0, y: 0 };
		let mut end = Coordinate { x: 0, y: 0 };
		let mut walls: HashSet<Coordinate> = HashSet::new();
		for (y, line) in input_string.lines().enumerate() {
			for (x, c) in line.chars().enumerate() {
				match c {
					'S' => start = Coordinate { x, y },
					'E' => end = Coordinate { x, y },
					'#' => {
						walls.insert(Coordinate { x, y });
					}
					_ => (),
				}
			}
		}

		(start, end, walls)
	};

	let mut states: BinaryHeap<Reverse<TravelState>> = BinaryHeap::new();
	let mut visited: HashMap<(Coordinate, Direction), u32> = HashMap::new();
	states.push(Reverse(TravelState {
		score_so_far: 0,
		facing_direction: Direction::default(),
		current_position: start.clone(),
		path_tiles: vec![start],
	}));

	let mut on_best_path: HashSet<Coordinate> = HashSet::new();

	let mut best_score = u32::MAX;
	while let Some(Reverse(state)) = states.pop() {
		if state.score_so_far > best_score {
			continue;
		}
		if state.current_position == end {
			if state.score_so_far < best_score {
				on_best_path.clear();
				best_score = state.score_so_far;
			}
			for coord in state.path_tiles {
				on_best_path.insert(coord);
			}
			continue;
		}
		let state_position_direction = (state.current_position.clone(), state.facing_direction);
		let visited_entry = visited.entry(state_position_direction);
		match visited_entry {
			Entry::Occupied(mut entry) => {
				let visit_score = entry.get_mut();
				if state.score_so_far > *visit_score {
					continue;
				}
				*visit_score = state.score_so_far;
			}
			Entry::Vacant(entry) => {
				entry.insert(state.score_so_far);
			}
		}
		let next_move_coord = state.current_position.advance_in_direction(state.facing_direction);
		if let Some(next_coord) = next_move_coord {
			if !walls.contains(&next_coord) {
				let score_so_far = state.score_so_far + 1;
				let current_position = next_coord;
				let facing_direction = state.facing_direction;
				let mut path_tiles = state.path_tiles.clone();
				path_tiles.push(current_position.clone());
				let next_state = TravelState {
					score_so_far,
					current_position,
					facing_direction,
					path_tiles,
				};
				states.push(Reverse(next_state));
			}
		}

		for facing_direction in state.facing_direction.get_adjacent() {
			let score_so_far = state.score_so_far + 1000;
			let current_position = state.current_position.clone();
			let path_tiles = state.path_tiles.clone();
			let next_state = TravelState {
				score_so_far,
				current_position,
				facing_direction,
				path_tiles,
			};
			states.push(Reverse(next_state));
		}
	}

	println!("{}", on_best_path.len());

	Ok(())
}
