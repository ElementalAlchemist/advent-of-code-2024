use std::collections::HashSet;
use std::error::Error;
use std::fs;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
	Up,
	Right,
	Down,
	Left,
}

impl Direction {
	fn next_after_turn(&self) -> Self {
		match self {
			Self::Up => Self::Right,
			Self::Right => Self::Down,
			Self::Down => Self::Left,
			Self::Left => Self::Up,
		}
	}
}

impl Default for Direction {
	fn default() -> Self {
		Self::Up
	}
}

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
struct Coordinate {
	x: usize,
	y: usize,
}

impl Coordinate {
	fn advance_in_direction(&self, direction: Direction, width: usize, height: usize) -> Option<Self> {
		match direction {
			Direction::Up => {
				if self.y == 0 {
					None
				} else {
					Some(Coordinate {
						x: self.x,
						y: self.y - 1,
					})
				}
			}
			Direction::Right => {
				let x = self.x + 1;
				if x >= width {
					None
				} else {
					Some(Coordinate { x, y: self.y })
				}
			}
			Direction::Down => {
				let y = self.y + 1;
				if y >= height {
					None
				} else {
					Some(Coordinate { x: self.x, y })
				}
			}
			Direction::Left => {
				if self.x == 0 {
					None
				} else {
					Some(Coordinate {
						x: self.x - 1,
						y: self.y,
					})
				}
			}
		}
	}
}

fn check_direction_for_loop(
	width: usize,
	height: usize,
	obstruction_positions: &HashSet<Coordinate>,
	visited_states: &HashSet<(Coordinate, Direction)>,
	mut current_position: Coordinate,
	mut current_direction: Direction,
) -> bool {
	let mut visited_states = visited_states.clone();
	visited_states.insert((current_position, current_direction));
	'walk: while let Some(mut next_position) = current_position.advance_in_direction(current_direction, width, height) {
		while obstruction_positions.contains(&next_position) {
			current_direction = current_direction.next_after_turn();
			let Some(new_next_position) = current_position.advance_in_direction(current_direction, width, height)
			else {
				break 'walk;
			};
			next_position = new_next_position;
		}
		current_position = next_position;
		if visited_states.contains(&(current_position, current_direction)) {
			return true;
		}
		visited_states.insert((current_position, current_direction));
	}
	false
}

fn state_contains_coordinate(visited_states: &HashSet<(Coordinate, Direction)>, coord: Coordinate) -> bool {
	visited_states.contains(&(coord, Direction::Up))
		|| visited_states.contains(&(coord, Direction::Right))
		|| visited_states.contains(&(coord, Direction::Down))
		|| visited_states.contains(&(coord, Direction::Left))
}

struct InitialState {
	guard_direction: Direction,
	guard_position: Coordinate,
	obstruction_positions: HashSet<Coordinate>,
	width: usize,
	height: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
	let initial_state = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut height = 0;
		let mut width = 0;
		let mut guard_position = Coordinate::default();
		let mut obstruction_positions: HashSet<Coordinate> = HashSet::new();
		for (y, line) in input_string.lines().enumerate() {
			height += 1;
			width = line.len();
			for (x, c) in line.chars().enumerate() {
				if c == '^' {
					guard_position = Coordinate { x, y };
				} else if c == '#' {
					obstruction_positions.insert(Coordinate { x, y });
				}
			}
		}

		InitialState {
			guard_direction: Direction::default(),
			guard_position,
			obstruction_positions,
			width,
			height,
		}
	};

	let mut visited_states: HashSet<(Coordinate, Direction)> = HashSet::new();
	let mut guard_position = initial_state.guard_position;
	let mut guard_direction = initial_state.guard_direction;
	let obstruction_positions = initial_state.obstruction_positions;
	let width = initial_state.width;
	let height = initial_state.height;
	let mut added_obstructions: HashSet<Coordinate> = HashSet::new();
	visited_states.insert((guard_position, guard_direction));
	let guard_start_position = guard_position;

	'walk: loop {
		let Some(mut next_position) = guard_position.advance_in_direction(guard_direction, width, height) else {
			break;
		};
		while obstruction_positions.contains(&next_position) {
			guard_direction = guard_direction.next_after_turn();
			let Some(new_next_position) = guard_position.advance_in_direction(guard_direction, width, height) else {
				break 'walk;
			};
			next_position = new_next_position;
		}

		if !state_contains_coordinate(&visited_states, next_position) {
			let mut obstructions_with_new = obstruction_positions.clone();
			obstructions_with_new.insert(next_position);
			if check_direction_for_loop(
				width,
				height,
				&obstructions_with_new,
				&visited_states,
				guard_position,
				guard_direction.next_after_turn(),
			) {
				added_obstructions.insert(next_position);
			}
		}
		guard_position = next_position;
		visited_states.insert((guard_position, guard_direction));
	}

	added_obstructions.remove(&guard_start_position);
	println!("{}", added_obstructions.len());

	Ok(())
}
