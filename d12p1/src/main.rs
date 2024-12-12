use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
	x: usize,
	y: usize,
}

impl Coordinate {
	fn get_adjacent(&self, width: usize, height: usize) -> Vec<Coordinate> {
		let mut adjacent = Vec::new();
		if self.x > 0 {
			adjacent.push(Coordinate {
				x: self.x - 1,
				y: self.y,
			});
		}
		if self.y > 0 {
			adjacent.push(Coordinate {
				x: self.x,
				y: self.y - 1,
			});
		}
		if self.x < width - 1 {
			adjacent.push(Coordinate {
				x: self.x + 1,
				y: self.y,
			});
		}
		if self.y < height - 1 {
			adjacent.push(Coordinate {
				x: self.x,
				y: self.y + 1,
			});
		}
		adjacent
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let (plots_by_type, width, height) = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut plots_by_type: HashMap<char, HashSet<Coordinate>> = HashMap::new();
		let mut width: usize = 0;
		let mut height: usize = 0;
		for (y, line) in input_string.lines().enumerate() {
			height += 1;
			width = line.len();
			for (x, c) in line.chars().enumerate() {
				plots_by_type.entry(c).or_default().insert(Coordinate { x, y });
			}
		}
		(plots_by_type, width, height)
	};

	let mut regions: Vec<HashSet<Coordinate>> = Vec::new();
	let mut handled_plots: HashSet<Coordinate> = HashSet::new();
	for plots in plots_by_type.values() {
		for plot in plots.iter() {
			if handled_plots.contains(plot) {
				continue;
			}
			let mut region = HashSet::new();
			region.insert(plot.clone());
			handled_plots.insert(plot.clone());

			let mut plots_to_handle = plot.get_adjacent(width, height);
			while let Some(next_plot) = plots_to_handle.pop() {
				if handled_plots.contains(&next_plot) {
					continue;
				}
				if !plots.contains(&next_plot) {
					continue;
				}
				for next_next_plot in next_plot.get_adjacent(width, height) {
					plots_to_handle.push(next_next_plot);
				}
				region.insert(next_plot.clone());
				handled_plots.insert(next_plot);
			}

			regions.push(region);
		}
	}

	let mut total_fence = 0;
	for region in regions {
		let region_area = region.len();
		let mut region_perimeter = 0;
		for plot in region.iter() {
			let adjacent_plots = plot.get_adjacent(width, height);
			region_perimeter += 4 - adjacent_plots.len();
			for adjacent_plot in adjacent_plots {
				if !region.contains(&adjacent_plot) {
					region_perimeter += 1;
				}
			}
		}
		total_fence += region_area * region_perimeter;
	}

	println!("{}", total_fence);

	Ok(())
}
