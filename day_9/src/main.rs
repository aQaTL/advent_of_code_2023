use anyhow::bail;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_9/input.txt")?;
	let (part_1, part_2) = solve(&input)?;
	println!("Part 1: {part_1}");
	println!("Part 2: {part_2}");
	Ok(())
}

fn solve(input: &str) -> anyhow::Result<(i64, i64)> {
	let lines: Vec<Vec<i64>> = input
		.trim()
		.lines()
		.map(|line| line.split(' ').map(|n| n.parse::<i64>()).try_collect())
		.try_collect()?;

	let (mut part_1, mut part_2) = (0, 0);
	for line in lines.into_iter() {
		let mut more_lines: Vec<Vec<i64>> = vec![line];

		loop {
			let Some(l) = more_lines.last() else {
				bail!("wtf");
			};
			if l.iter().all(|n| *n == 0) {
				break;
			}
			let new_l: Vec<_> = l.iter().tuple_windows().map(|(a, b)| b - a).collect();
			more_lines.push(new_l);
		}

		part_1 += more_lines
			.iter()
			.map(|line| line.last().copied().unwrap_or_default())
			.rev()
			.skip(1)
			.sum::<i64>();

		part_2 += more_lines
			.iter()
			.map(|line| line.first().copied().unwrap_or_default())
			.rev()
			.skip(1)
			.fold(0, |prev_first_n, first_n| first_n - prev_first_n);
	}

	Ok((part_1, part_2))
}

#[cfg(test)]
mod tests {
	const EXAMPLE_1: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

	#[test]
	fn part_1() {
		assert_eq!(super::solve(EXAMPLE_1).unwrap().0, 114);
	}

	#[test]
	fn part_2() {
		assert_eq!(super::solve(EXAMPLE_1).unwrap().1, 2);
	}
}
