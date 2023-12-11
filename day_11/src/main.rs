use itertools::Itertools;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_11/input.txt")?;
	let (part_1, part_2) = solve(&input)?;
	println!("Part 1: {part_1}");
	println!("Part 2: {part_2}");
	Ok(())
}

fn solve(input: &str) -> anyhow::Result<(i64, i64)> {
	let grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();

	// Rows and columns converted to true if they have a galaxy and false otherwise
	let rows: Vec<bool> = grid
		.iter()
		.map(|row| row.iter().any(|c| *c == b'#'))
		.collect();
	let columns: Vec<bool> = (0..grid[0].len())
		.map(|column_idx| grid.iter().any(|row| row[column_idx] == b'#'))
		.collect();

	let mut galaxies = Vec::new();
	for (y, row) in grid.iter().enumerate() {
		for (x, b) in row.iter().enumerate() {
			if *b == b'#' {
				galaxies.push((x, y));
			}
		}
	}

	let (mut sum_p1, mut sum_p2) = (0, 0);
	for (a, b) in galaxies.iter().copied().tuple_combinations() {
		let x_expand = (a.0.min(b.0)..a.0.max(b.0))
			.filter(|x| !columns[*x])
			.count();
		let y_expand = (a.1.min(b.1)..a.1.max(b.1)).filter(|y| !rows[*y]).count();

		let (mut a_p1, mut b_p1) = (a, b);
		let (mut a_p2, mut b_p2) = (a, b);
		if a.0 > b.0 {
			a_p1.0 += x_expand;
			a_p2.0 += x_expand * (1_000_000 - 1);
		} else {
			b_p1.0 += x_expand;
			b_p2.0 += x_expand * (1_000_000 - 1);
		}
		if a.1 > b.1 {
			a_p1.1 += y_expand;
			a_p2.1 += y_expand * (1_000_000 - 1);
		} else {
			b_p1.1 += y_expand;
			b_p2.1 += y_expand * (1_000_000 - 1);
		}

		sum_p1 += manhattan_distance(a_p1, b_p1);
		sum_p2 += manhattan_distance(a_p2, b_p2);
	}

	Ok((sum_p1, sum_p2))
}

fn manhattan_distance((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> i64 {
	(x1 as i64 - x2 as i64).abs() + (y1 as i64 - y2 as i64).abs()
}

#[cfg(test)]
mod tests {
	const EXAMPLE_1: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

	#[test]
	fn part_1() {
		assert_eq!(super::solve(EXAMPLE_1).unwrap().0, 374);
	}
}
