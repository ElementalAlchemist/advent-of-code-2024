use std::collections::VecDeque;
use std::error::Error;
use std::fs;

enum BlockType {
	Occupied(usize),
	Free,
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut disk_layout = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut used_block = true;
		let mut disk_layout: VecDeque<BlockType> = VecDeque::new();
		let mut file_id = 0;
		for c in input_string.chars() {
			let number_of_blocks: usize = (c as usize) - 48;
			for _ in 0..number_of_blocks {
				if used_block {
					disk_layout.push_back(BlockType::Occupied(file_id));
				} else {
					disk_layout.push_back(BlockType::Free);
				}
			}
			if used_block {
				used_block = false;
				file_id += 1;
			} else {
				used_block = true;
			}
		}

		disk_layout
	};

	let mut compact_disk: Vec<usize> = Vec::new();
	while let Some(current_sector) = disk_layout.pop_front() {
		match current_sector {
			BlockType::Occupied(file_id) => compact_disk.push(file_id),
			BlockType::Free => {
				while let Some(back_sector) = disk_layout.pop_back() {
					match back_sector {
						BlockType::Occupied(file_id) => {
							compact_disk.push(file_id);
							break;
						}
						BlockType::Free => (),
					}
				}
			}
		}
	}

	let checksum: usize = compact_disk.into_iter().enumerate().map(|(index, id)| index * id).sum();
	println!("{}", checksum);

	Ok(())
}
