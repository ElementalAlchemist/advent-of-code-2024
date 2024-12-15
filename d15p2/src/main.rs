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

enum ObjectType {
	Box,
	BoxMore,
	Wall,
}

#[derive(Clone, Copy)]
enum Direction {
	Up,
	Right,
	Down,
	Left,
}

impl Direction {
	fn is_vertical(&self) -> bool {
		matches!(self, Self::Up | Self::Down)
	}
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
				let x = x * 2;
				match c {
					'@' => {
						robot_coordinate = Some(Coordinate { x, y });
					}
					'O' => {
						coordinates.insert(Coordinate { x, y }, ObjectType::Box);
						coordinates.insert(Coordinate { x: x + 1, y }, ObjectType::BoxMore);
					}
					'#' => {
						coordinates.insert(Coordinate { x, y }, ObjectType::Wall);
						coordinates.insert(Coordinate { x: x + 1, y }, ObjectType::Wall);
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
		let mut box_coordinates: HashMap<Coordinate, ObjectType> = HashMap::new();
		let mut looking_coordinates = vec![robot_coordinate.clone()];
		while !looking_coordinates.is_empty() {
			let mut new_looking_coordinates = Vec::new();
			for looking_coordinate in looking_coordinates {
				let next_coordinate = looking_coordinate.next_coordinate(robot_move);
				if let Some(next_coord_object) = warehouse_coordinates.get(&next_coordinate) {
					match next_coord_object {
						ObjectType::Box => {
							box_coordinates.insert(next_coordinate.clone(), ObjectType::Box);
							new_looking_coordinates.push(next_coordinate.clone());
							if robot_move.is_vertical() {
								let other_next_coordinate = next_coordinate.next_coordinate(Direction::Right);
								box_coordinates.insert(other_next_coordinate.clone(), ObjectType::BoxMore);
								new_looking_coordinates.push(other_next_coordinate);
							}
						}
						ObjectType::BoxMore => {
							box_coordinates.insert(next_coordinate.clone(), ObjectType::BoxMore);
							new_looking_coordinates.push(next_coordinate.clone());
							if robot_move.is_vertical() {
								let other_next_coordinate = next_coordinate.next_coordinate(Direction::Left);
								box_coordinates.insert(other_next_coordinate.clone(), ObjectType::Box);
								new_looking_coordinates.push(other_next_coordinate);
							}
						}
						ObjectType::Wall => continue 'moves,
					}
				}
			}
			looking_coordinates = new_looking_coordinates;
		}
		robot_coordinate = robot_coordinate.next_coordinate(robot_move);
		for coord in box_coordinates.keys() {
			warehouse_coordinates.remove(coord);
		}
		for (box_coord, box_type) in box_coordinates {
			warehouse_coordinates.insert(box_coord.next_coordinate(robot_move), box_type);
		}
	}

	let mut gps_coord_sum = 0;
	for (coordinate, object) in warehouse_coordinates.iter() {
		if let ObjectType::Box = object {
			gps_coord_sum += coordinate.gps();
		}
	}
	println!("{}", gps_coord_sum);

	Ok(())
}
