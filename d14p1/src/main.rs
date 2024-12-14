use std::cmp::Ordering;
use std::error::Error;
use std::fs;

const AREA_WIDTH: i32 = 101;
const AREA_HEIGHT: i32 = 103;
const SECONDS: i32 = 100;

struct Coordinate {
	x: i32,
	y: i32,
}

struct Robot {
	position: Coordinate,
	velocity: Coordinate,
}

fn main() -> Result<(), Box<dyn Error>> {
	let robots = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut robots = Vec::new();
		for line in input_string.lines() {
			let line = line.strip_prefix("p=").expect("Line starts with p=");
			let (position, velocity) = line.split_once(" v=").expect("Line has v=");
			let (position_x, position_y) = position.split_once(',').expect("Position is coordinate");
			let (velocity_x, velocity_y) = velocity.split_once(',').expect("Velocity is coordinate");
			let position_x: i32 = position_x.parse()?;
			let position_y: i32 = position_y.parse()?;
			let velocity_x: i32 = velocity_x.parse()?;
			let velocity_y: i32 = velocity_y.parse()?;
			robots.push(Robot {
				position: Coordinate {
					x: position_x,
					y: position_y,
				},
				velocity: Coordinate {
					x: velocity_x,
					y: velocity_y,
				},
			});
		}

		robots
	};

	let mut robot_coordinates: Vec<Coordinate> = Vec::with_capacity(robots.len());
	for robot in robots {
		let x = (robot.position.x + (robot.velocity.x * SECONDS)).rem_euclid(AREA_WIDTH);
		let y = (robot.position.y + (robot.velocity.y * SECONDS)).rem_euclid(AREA_HEIGHT);
		robot_coordinates.push(Coordinate { x, y });
	}

	let x_divide = AREA_WIDTH / 2;
	let y_divide = AREA_HEIGHT / 2;
	let mut top_left_quad = 0;
	let mut top_right_quad = 0;
	let mut bottom_left_quad = 0;
	let mut bottom_right_quad = 0;
	for coord in robot_coordinates {
		match (coord.x.cmp(&x_divide), coord.y.cmp(&y_divide)) {
			(Ordering::Less, Ordering::Less) => top_left_quad += 1,
			(Ordering::Greater, Ordering::Less) => top_right_quad += 1,
			(Ordering::Less, Ordering::Greater) => bottom_left_quad += 1,
			(Ordering::Greater, Ordering::Greater) => bottom_right_quad += 1,
			_ => (),
		}
	}

	let safety_factor = top_left_quad * top_right_quad * bottom_left_quad * bottom_right_quad;
	println!("{}", safety_factor);

	Ok(())
}
