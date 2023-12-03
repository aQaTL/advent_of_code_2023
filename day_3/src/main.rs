#![allow(clippy::needless_range_loop)]

use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_3/input.txt")?;
	let (part_1, part_2) = solve(&input)?;
	println!("Part 1: {}", part_1);
	println!("Part 2: {}", part_2);
	Ok(())
}

fn solve(input: &str) -> anyhow::Result<(i64, i64)> {
	let schematic: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();

	let mut nums = HashMap::<(usize, usize), Vec<u8>>::new();

	for y in 0..schematic.len() {
		let mut x = 0;
		while x < schematic[y].len() {
			let b = schematic[y][x];
			if !b.is_ascii_digit() {
				x += 1;
				continue;
			}

			let end_idx = schematic[y]
				.iter()
				.enumerate()
				.skip(x)
				.find(|(_, b)| !b.is_ascii_digit())
				.map(|(idx, _)| idx)
				.unwrap_or(schematic[y].len());
			let number = schematic[y][x..end_idx].to_vec();
			nums.insert((y, x), number);
			x = end_idx;
		}
	}

	let mut gears = HashMap::<(usize, usize), Vec<i64>>::new();

	let mut part_1 = 0;
	for ((y, x), number) in nums {
		let n = std::str::from_utf8(&number).unwrap().parse::<i64>()?;
		// (y, x)
		let dirs = [
			(-1, -1),
			(-1, 0),
			(-1, 1),
			(0, 1),
			(1, 1),
			(1, 0),
			(1, -1),
			(0, -1),
		];
		let mut matched = false;
		'current_num: for b_idx in 0..number.len() {
			let x = x + b_idx;
			for (d_y, d_x) in dirs {
				let (new_y, new_x) = (y as isize + d_y, x as isize + d_x);
				if new_y < 0
					|| new_y >= schematic.len() as isize
					|| new_x < 0 || new_x >= schematic[0].len() as isize
				{
					continue;
				}
				let (new_y, new_x) = (new_y as usize, new_x as usize);
				let b = schematic[new_y][new_x];
				if !matches!(b, b'0'..=b'9' | b'.') {
					matched = true;
				}
				if b == b'*' {
					gears.entry((new_y, new_x)).or_default().push(n);
					break 'current_num;
				}
			}
		}
		if matched {
			part_1 += n;
		}
	}

	let part_2 = gears
		.into_values()
		.filter(|nums| nums.len() == 2)
		.map(|nums| nums.into_iter().product::<i64>())
		.sum::<i64>();

	Ok((part_1, part_2))
}

#[cfg(test)]
mod tests {
	const EXAMPLE_1: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
	#[test]
	fn test() {
		let (part_1, part_2) = super::solve(EXAMPLE_1).unwrap();
		assert_eq!(part_1, 4361);
		assert_eq!(part_2, 467835);
	}
}
