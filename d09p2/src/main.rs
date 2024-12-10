use std::error::Error;
use std::fs;

#[derive(Debug, Eq, PartialEq)]
enum BlockType {
	Occupied(usize),
	Free,
}

enum LayoutGroupDescription {
	Occupied(usize, usize),
	Free(usize),
}

fn main() -> Result<(), Box<dyn Error>> {
	let disk_layout = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut used_block = true;
		let mut disk_layout: Vec<LayoutGroupDescription> = Vec::new();
		let mut file_id = 0;
		for c in input_string.chars() {
			let number_of_blocks: usize = (c as usize) - 48;
			if used_block {
				disk_layout.push(LayoutGroupDescription::Occupied(file_id, number_of_blocks));
				used_block = false;
				file_id += 1;
			} else {
				disk_layout.push(LayoutGroupDescription::Free(number_of_blocks));
				used_block = true;
			}
		}

		disk_layout
	};

	let mut files: Vec<(usize, usize, usize)> = Vec::new();
	let mut free_space_blocks: Vec<(usize, usize)> = Vec::new();
	let mut position = 0;
	for block_group in disk_layout.iter() {
		match block_group {
			LayoutGroupDescription::Occupied(file_id, number_of_blocks) => {
				files.push((*file_id, position, *number_of_blocks));
				position += *number_of_blocks;
			}
			LayoutGroupDescription::Free(number_of_blocks) => {
				free_space_blocks.push((position, *number_of_blocks));
				position += *number_of_blocks;
			}
		}
	}

	for (_, file_start, file_size) in files.iter_mut().rev() {
		for (free_space_start, free_space_size) in free_space_blocks.iter_mut() {
			if *free_space_start > *file_start {
				break;
			}
			if *free_space_size < *file_size {
				continue;
			}
			let remaining_free_space_size = *free_space_size - *file_size;
			*file_start = *free_space_start;
			*free_space_size = remaining_free_space_size;
			*free_space_start += *file_size;
			break;
		}
	}

	let mut compact_disk: Vec<BlockType> = Vec::new();
	for (file_id, file_start, file_size) in files {
		while compact_disk.len() < file_start + file_size {
			compact_disk.push(BlockType::Free);
		}
		for block in compact_disk.iter_mut().skip(file_start).take(file_size) {
			assert_eq!(*block, BlockType::Free);
			*block = BlockType::Occupied(file_id);
		}
	}

	let checksum: usize = compact_disk
		.into_iter()
		.enumerate()
		.filter_map(|(index, block_type)| match block_type {
			BlockType::Occupied(id) => Some(index * id),
			BlockType::Free => None,
		})
		.sum();
	println!("{}", checksum);

	Ok(())
}
