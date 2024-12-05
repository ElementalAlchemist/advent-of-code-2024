use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	let (page_dependencies, page_lists_to_print) = {
		let input_string = fs::read_to_string("input.txt")?;

		let mut input_lines = input_string.lines();

		let mut page_dependencies: HashMap<u32, HashSet<u32>> = HashMap::new();
		for line in input_lines.by_ref() {
			if line.is_empty() {
				break;
			}
			let (print_first, print_later) = line.split_once('|').expect("Delimiter found");
			let print_first: u32 = print_first.parse()?;
			let print_later: u32 = print_later.parse()?;
			page_dependencies.entry(print_later).or_default().insert(print_first);
		}

		let mut pages_lists: Vec<Vec<u32>> = Vec::new();
		for line in input_lines {
			let pages: Vec<u32> = line.split(',').map(|page| page.parse().expect("Page number")).collect();
			pages_lists.push(pages);
		}

		(page_dependencies, pages_lists)
	};

	let mut middle_page_sum = 0;
	for page_list in page_lists_to_print {
		let included_pages: HashSet<u32> = page_list.iter().copied().collect();
		let mut found_pages: HashSet<u32> = HashSet::new();
		let mut needs_update = false;
		for page in page_list.iter() {
			if let Some(required_before) = page_dependencies.get(page) {
				let required_included_pages: HashSet<u32> =
					included_pages.intersection(required_before).copied().collect();
				let required_found_pages: HashSet<u32> = found_pages.intersection(required_before).copied().collect();
				if required_included_pages != required_found_pages {
					needs_update = true;
					break;
				}
			}
			found_pages.insert(*page);
		}
		if !needs_update {
			continue;
		}

		let mut new_page_list: Vec<u32> = Vec::new();
		let mut newly_ordered_pages: HashSet<u32> = HashSet::new();
		let mut list_page_dependencies: HashMap<u32, HashSet<u32>> = HashMap::new();
		for page in page_list.iter() {
			if let Some(dependencies) = page_dependencies.get(page) {
				list_page_dependencies.insert(*page, included_pages.intersection(dependencies).copied().collect());
			}
		}
		while new_page_list.len() < page_list.len() {
			for page in page_list.iter() {
				if newly_ordered_pages.contains(page) {
					continue;
				}
				if let Some(dependencies) = list_page_dependencies.get(page) {
					if newly_ordered_pages.is_superset(dependencies) {
						new_page_list.push(*page);
						newly_ordered_pages.insert(*page);
					}
				} else {
					new_page_list.push(*page);
					newly_ordered_pages.insert(*page);
				}
			}
		}

		let middle_page = new_page_list[new_page_list.len() / 2];
		middle_page_sum += middle_page;
	}

	println!("{}", middle_page_sum);

	Ok(())
}
