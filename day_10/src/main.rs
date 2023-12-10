#![allow(clippy::ptr_arg)]

use anyhow::bail;
use itertools::Itertools;
use std::collections::{hash_map, HashMap, VecDeque};
use std::ops::Div;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_10/input.txt")?;
	let (part_1, part_2) = solve(&input)?;
	println!("Part 1: {part_1}");
	println!("Part 2: {part_2}");
	Ok(())
}

fn solve(input: &str) -> anyhow::Result<(i64, i64)> {
	let grid: Vec<Vec<Tile>> = input
		.lines()
		.map(|line| line.bytes().map(Tile::try_from).try_collect())
		.try_collect()?;

	let mut lengths = HashMap::new();

	let starting_pos: (usize, usize) = grid
		.iter()
		.enumerate()
		.find_map(|(y_idx, line)| {
			line.iter()
				.enumerate()
				.find(|(_, tile)| **tile == Tile::StartingPos)
				.map(|(x_idx, _)| (y_idx, x_idx))
		})
		.unwrap();

	lengths.insert(starting_pos, 0);

	let mut queue = VecDeque::new();
	queue.push_back((starting_pos, 0));

	while let Some((pos, length)) = queue.pop_front() {
		let fns = [get_north, get_east, get_south, get_west];
		for f in fns {
			if let Some((next_pos, _next_tile)) = f(&grid, pos) {
				if let hash_map::Entry::Vacant(entry) = lengths.entry(next_pos) {
					entry.insert(length + 1);
					queue.push_back((next_pos, length + 1));
				}
			}
		}
	}

	let part_1 = lengths.values().max().copied().unwrap();

	lengths.clear();
	lengths.insert(starting_pos, 0);
	queue.push_back((starting_pos, 0));

	'queue_loop: while let Some((pos, length)) = queue.pop_front() {
		let fns = [get_north, get_east, get_south, get_west];
		for f in fns {
			if let Some((next_pos, _next_tile)) = f(&grid, pos) {
				if let hash_map::Entry::Vacant(entry) = lengths.entry(next_pos) {
					entry.insert(length + 1);
					queue.push_back((next_pos, length + 1));
					continue 'queue_loop;
				}
			}
		}
	}

	let points: Vec<(usize, usize)> = lengths
		.into_iter()
		.sorted_by_key(|(_pos, length)| *length)
		.map(|(pos, _)| pos)
		.collect();

	let area = polygon_area(&points);
	// Use the polygon area from shoelace formula to calculate number of internal points using
	// Pick's Theorem (https://en.wikipedia.org/wiki/Pick%27s_theorem)
	let part_2 = internal_points(points.len() as i64, area);

	Ok((part_1, part_2))
}

// https://en.wikipedia.org/wiki/Shoelace_formula
fn polygon_area(points: &[(usize, usize)]) -> i64 {
	points
		.iter()
		.zip(points.iter().cycle().skip(points.len() - 1))
		.map(|((i_y, i_x), (j_y, j_x))| (*j_x as i64 + *i_x as i64) * (*j_y as i64 - *i_y as i64))
		.sum::<i64>()
		.div(2)
		.abs()
}

fn internal_points(boundary_points: i64, polygon_area: i64) -> i64 {
	-(boundary_points / 2) + 1 + polygon_area
}

fn get_north(
	grid: &Vec<Vec<Tile>>,
	(pos_y, pos_x): (usize, usize),
) -> Option<((usize, usize), Tile)> {
	if pos_y == 0
		|| matches!(
			grid[pos_y][pos_x],
			Tile::Horizontal | Tile::BendSouthWest | Tile::BendSouthEast | Tile::Ground
		) {
		return None;
	}
	let tile = grid[pos_y - 1][pos_x];
	if matches!(
		tile,
		Tile::Horizontal | Tile::BendNorthEast | Tile::BendNorthWest | Tile::Ground
	) {
		return None;
	}
	Some(((pos_y - 1, pos_x), tile))
}

fn get_east(
	grid: &Vec<Vec<Tile>>,
	(pos_y, pos_x): (usize, usize),
) -> Option<((usize, usize), Tile)> {
	if matches!(
		grid[pos_y][pos_x],
		Tile::Vertical | Tile::BendSouthWest | Tile::BendNorthWest | Tile::Ground
	) {
		return None;
	}
	let tile = *grid[pos_y].get(pos_x + 1)?;
	if matches!(
		tile,
		Tile::Vertical | Tile::BendNorthEast | Tile::BendSouthEast | Tile::Ground
	) {
		return None;
	}
	Some(((pos_y, pos_x + 1), tile))
}

fn get_south(
	grid: &Vec<Vec<Tile>>,
	(pos_y, pos_x): (usize, usize),
) -> Option<((usize, usize), Tile)> {
	if matches!(
		grid[pos_y][pos_x],
		Tile::Horizontal | Tile::BendNorthEast | Tile::BendNorthWest | Tile::Ground
	) {
		return None;
	}
	let tile = grid.get(pos_y + 1)?[pos_x];
	if matches!(
		tile,
		Tile::Horizontal | Tile::BendSouthEast | Tile::BendSouthWest | Tile::Ground
	) {
		return None;
	}
	Some(((pos_y + 1, pos_x), tile))
}

fn get_west(
	grid: &Vec<Vec<Tile>>,
	(pos_y, pos_x): (usize, usize),
) -> Option<((usize, usize), Tile)> {
	if pos_x == 0
		|| matches!(
			grid[pos_y][pos_x],
			Tile::Vertical | Tile::BendNorthEast | Tile::BendSouthEast | Tile::Ground
		) {
		return None;
	}
	let tile = grid[pos_y][pos_x - 1];
	if matches!(
		tile,
		Tile::Vertical | Tile::BendNorthWest | Tile::BendSouthWest | Tile::Ground
	) {
		return None;
	}
	Some(((pos_y, pos_x - 1), tile))
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
enum Tile {
	Vertical = b'|',
	Horizontal = b'-',
	BendNorthEast = b'L',
	BendNorthWest = b'J',
	BendSouthWest = b'7',
	BendSouthEast = b'F',
	Ground = b'.',
	StartingPos = b'S',
}

impl TryFrom<u8> for Tile {
	type Error = anyhow::Error;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		if matches!(value, b'|' | b'-' | b'L' | b'J' | b'7' | b'F' | b'.' | b'S') {
			Ok(unsafe { std::mem::transmute(value) })
		} else {
			bail!("invalid tile value {}", value as char)
		}
	}
}

#[cfg(test)]
mod tests {
	const EXAMPLE_1: &str = ".....
.S-7.
.|.|.
.L-J.
.....
";

	const EXAMPLE_2: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";

	const EXAMPLE_3: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

	const EXAMPLE_4: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";

	#[test]
	fn part_1_example_1() {
		assert_eq!(super::solve(EXAMPLE_1).unwrap().0, 4);
	}

	#[test]
	fn part_1_example_2() {
		assert_eq!(super::solve(EXAMPLE_2).unwrap().0, 4);
	}

	#[test]
	fn part_1_example_3() {
		assert_eq!(super::solve(EXAMPLE_3).unwrap().0, 8);
	}

	#[test]
	fn part_1_example_4() {
		assert_eq!(super::solve(EXAMPLE_4).unwrap().0, 8);
	}

	const EXAMPLE_5: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

	const EXAMPLE_6: &str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
";

	const EXAMPLE_7: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

	const EXAMPLE_8: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

	#[test]
	fn part_2_example_1() {
		assert_eq!(super::solve(EXAMPLE_1).unwrap().1, 1);
	}

	#[test]
	fn part_2_example_5() {
		assert_eq!(super::solve(EXAMPLE_5).unwrap().1, 4);
	}

	#[test]
	fn part_2_example_6() {
		assert_eq!(super::solve(EXAMPLE_6).unwrap().1, 4);
	}

	#[test]
	fn part_2_example_7() {
		assert_eq!(super::solve(EXAMPLE_7).unwrap().1, 8);
	}

	#[test]
	fn part_2_example_8() {
		assert_eq!(super::solve(EXAMPLE_8).unwrap().1, 10);
	}
}
