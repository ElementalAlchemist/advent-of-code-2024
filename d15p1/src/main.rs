use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
	x: usize,
	y: usize,
}

impl Coordinate {
	fn next_coordinate(&self, direction: Direction) -> Self {
		match direction {
			Direction::Up => Self {
				x: self.x,
				y: self.y - 1,
			},
			Direction::Right => Self {
				x: self.x + 1,
				y: self.y,
			},
			Direction::Down => Self {
				x: self.x,
				y: self.y + 1,
			},
			Direction::Left => Self {
				x: self.x - 1,
				y: self.y,
			},
		}
	}

	fn gps(&self) -> usize {
		self.x + self.y * 100
	}
}

#[derive(Eq, PartialEq)]
enum ObjectType {
	Box,
	Wall,
}

#[derive(Clone, Copy)]
enum Direction {
	Up,
	Right,
	Down,
	Left,
}

fn main() -> Result<(), Box<dyn Error>> {
	let (mut warehouse_coordinates, mut robot_coordinate, robot_moves) = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut coordinates: HashMap<Coordinate, ObjectType> = HashMap::new();
		let mut robot_coordinate: Option<Coordinate> = None;
		let mut lines_iter = input_string.lines();
		for (y, line) in lines_iter.by_ref().enumerate() {
			if line.is_empty() {
				break;
			}
			for (x, c) in line.chars().enumerate() {
				match c {
					'@' => {
						robot_coordinate = Some(Coordinate { x, y });
					}
					'O' => {
						coordinates.insert(Coordinate { x, y }, ObjectType::Box);
					}
					'#' => {
						coordinates.insert(Coordinate { x, y }, ObjectType::Wall);
					}
					_ => (),
				}
			}
		}

		let mut moves: Vec<Direction> = Vec::new();
		for line in lines_iter {
			for c in line.chars() {
				let direction = match c {
					'^' => Direction::Up,
					'>' => Direction::Right,
					'v' => Direction::Down,
					'<' => Direction::Left,
					_ => unimplemented!(),
				};
				moves.push(direction);
			}
		}

		(coordinates, robot_coordinate.unwrap(), moves)
	};

	'moves: for robot_move in robot_moves {
		let mut box_coordinates: Vec<Coordinate> = Vec::new();
		let mut looking_coordinate = robot_coordinate.clone();
		loop {
			let next_coordinate = looking_coordinate.next_coordinate(robot_move);
			if let Some(next_coord_object) = warehouse_coordinates.get(&next_coordinate) {
				match next_coord_object {
					ObjectType::Box => {
						box_coordinates.push(next_coordinate.clone());
						looking_coordinate = next_coordinate;
					}
					ObjectType::Wall => continue 'moves,
				}
			} else {
				break;
			}
		}
		robot_coordinate = robot_coordinate.next_coordinate(robot_move);
		for coord in box_coordinates.iter() {
			warehouse_coordinates.remove(coord);
		}
		for coord in box_coordinates {
			warehouse_coordinates.insert(coord.next_coordinate(robot_move), ObjectType::Box);
		}
	}

	let mut gps_coord_sum = 0;
	for (coordinate, object) in warehouse_coordinates.iter() {
		if *object == ObjectType::Box {
			gps_coord_sum += coordinate.gps();
		}
	}
	println!("{}", gps_coord_sum);

	Ok(())
}