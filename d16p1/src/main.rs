use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
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
}

impl Ord for TravelState {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.score_so_far
			.cmp(&other.score_so_far)
			.then_with(|| self.current_position.cmp(&other.current_position))
			.then_with(|| self.facing_direction.cmp(&other.facing_direction))
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
	let mut visited: HashSet<(Coordinate, Direction)> = HashSet::new();
	states.push(Reverse(TravelState {
		score_so_far: 0,
		facing_direction: Direction::default(),
		current_position: start,
	}));

	let mut final_score = 0;
	while let Some(Reverse(state)) = states.pop() {
		if state.current_position == end {
			final_score = state.score_so_far;
			break;
		}
		let state_position_direction = (state.current_position.clone(), state.facing_direction);
		if visited.contains(&state_position_direction) {
			continue;
		}
		visited.insert(state_position_direction);
		let next_move_coord = state.current_position.advance_in_direction(state.facing_direction);
		if let Some(next_coord) = next_move_coord {
			if !walls.contains(&next_coord) {
				let score_so_far = state.score_so_far + 1;
				let current_position = next_coord;
				let facing_direction = state.facing_direction;
				let next_state = TravelState {
					score_so_far,
					current_position,
					facing_direction,
				};
				states.push(Reverse(next_state));
			}
		}

		for facing_direction in state.facing_direction.get_adjacent() {
			let score_so_far = state.score_so_far + 1000;
			let current_position = state.current_position.clone();
			let next_state = TravelState {
				score_so_far,
				current_position,
				facing_direction,
			};
			states.push(Reverse(next_state));
		}
	}

	println!("{}", final_score);

	Ok(())
}
