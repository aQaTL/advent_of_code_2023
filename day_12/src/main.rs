use anyhow::bail;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Write};

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_12/input.txt")?;
	// println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> anyhow::Result<i64> {
	let lines: Vec<(Vec<Spring>, Vec<i64>)> = input
		.lines()
		.map(|line| {
			let (springs, group_sizes) = line.split(' ').next_tuple().unwrap();
			let springs = springs.chars().map(Spring::try_from).try_collect()?;
			let group_sizes = group_sizes
				.split(',')
				.map(|x| x.parse::<i64>())
				.try_collect()?;
			Ok::<_, anyhow::Error>((springs, group_sizes))
		})
		.try_collect()?;

	let mut out = 0;
	let mut spring_idx = 0;
	for (spring, sizes) in lines {
		println!("{spring_idx}");
		spring_idx += 1;

		let unknowns: Vec<usize> = spring
			.iter()
			.enumerate()
			.filter(|(_, s)| **s == Spring::Unknown)
			.map(|(idx, _)| idx)
			.collect();

		out += solve(spring, &unknowns, &sizes);
	}

	Ok(out)
}

fn part_2(input: &str) -> anyhow::Result<i64> {
	let lines: Vec<(Vec<Spring>, Vec<i64>)> = input
		.lines()
		.map(|line| {
			let (springs, group_sizes) = line.split(' ').next_tuple().unwrap();
			let mut springs: Vec<Spring> = springs.chars().map(Spring::try_from).try_collect()?;
			let orig_springs = springs.clone();
			for _ in 0..4 {
				springs.push(Spring::Unknown);
				springs.extend_from_slice(orig_springs.as_slice());
			}

			let group_sizes = group_sizes
				.split(',')
				.map(|x| x.parse::<i64>())
				.try_collect::<_, Vec<i64>, _>()?
				.repeat(5);
			Ok::<_, anyhow::Error>((springs, group_sizes))
		})
		.try_collect()?;

	// let mut out = 0;
	let out = lines
		.into_par_iter()
		.enumerate()
		.map(|(idx, (spring, sizes))| {
			let unknowns: Vec<usize> = spring
				.iter()
				.enumerate()
				.filter(|(_, s)| **s == Spring::Unknown)
				.map(|(idx, _)| idx)
				.collect();

			let r = solve(spring, &unknowns, &sizes);
			println!("{idx} = {r}");
			r
		})
		.sum();

	Ok(out)
}

fn solve(mut spring: Vec<Spring>, unknowns: &[usize], sizes: &[i64]) -> i64 {
	let damaged_required: i64 = sizes.iter().copied().sum();
	if spring
		.iter()
		.filter(|s| matches!(**s, Spring::Damaged | Spring::Unknown))
		.count() < damaged_required as usize
	{
		return 0;
	}

	if unknowns.is_empty() {
		return if is_valid(spring, sizes) { 1 } else { 0 };
	} else {
		if already_invalid(&spring, sizes) {
			return 0;
		}
	}
	let mut out = 0;
	// for unknown_idx in unknowns.iter().copied() {
	let mut spring2 = spring.clone();

	spring[unknowns[0]] = Spring::Operational;
	out += solve(spring, &unknowns[1..], sizes);

	spring2[unknowns[0]] = Spring::Damaged;
	out += solve(spring2, &unknowns[1..], sizes);
	// }
	out
}

fn is_valid(spring: Vec<Spring>, sizes: &[i64]) -> bool {
	// println!("testing {spring:?} ");
	let mut spring_idx = 0;
	let mut current_size_idx = 0;
	while spring_idx < spring.len() {
		match spring[spring_idx] {
			Spring::Unknown => panic!("wtf"),
			Spring::Damaged => {
				let current_spring_idx = spring_idx;
				'searcher: while spring_idx < spring.len() {
					match spring[spring_idx] {
						Spring::Operational => break 'searcher,
						Spring::Damaged => (),
						Spring::Unknown => panic!("wtff"),
					}
					spring_idx += 1;
				}
				let damaged_size = spring_idx - current_spring_idx;

				match sizes.get(current_size_idx).copied() {
					None => {
						// println!("FAIL");
						return false;
					}
					Some(size) if size != (damaged_size as i64) => {
						// println!("FAILL");
						return false;
					}
					Some(_) => {
						current_size_idx += 1;
						continue;
					}
				}
			}
			Spring::Operational => (),
		}

		spring_idx += 1;
	}
	let r = current_size_idx == sizes.len();
	// if r {
	// 	println!("OK {current_size_idx} | {}", sizes.len());
	// } else {
	// 	println!("FAIL3 {current_size_idx} | {}", sizes.len());
	// }
	r
}

fn already_invalid(spring: &[Spring], sizes: &[i64]) -> bool {
	// println!("testing {spring:?} ");
	let mut spring_idx = 0;
	let mut current_size_idx = 0;
	while spring_idx < spring.len() {
		match spring[spring_idx] {
			Spring::Unknown => return false,
			Spring::Damaged => {
				let current_spring_idx = spring_idx;
				'searcher: while spring_idx < spring.len() {
					match spring[spring_idx] {
						Spring::Operational => break 'searcher,
						Spring::Damaged => (),
						Spring::Unknown => return false,
					}
					spring_idx += 1;
				}
				let damaged_size = spring_idx - current_spring_idx;

				match sizes.get(current_size_idx).copied() {
					None => {
						// println!("FAIL");
						return true;
					}
					Some(size) if size != (damaged_size as i64) => {
						// println!("FAILL");
						return true;
					}
					Some(_) => {
						current_size_idx += 1;
						continue;
					}
				}
			}
			Spring::Operational => (),
		}

		spring_idx += 1;
	}
	let r = current_size_idx != sizes.len();
	r
}

// 01234
// .###.

// ..?..?..# 1,1

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
enum Spring {
	Operational = b'.',
	Damaged = b'#',
	Unknown = b'?',
}

impl TryFrom<char> for Spring {
	type Error = anyhow::Error;

	fn try_from(c: char) -> Result<Self, Self::Error> {
		match c {
			'.' | '#' | '?' => unsafe { Ok(std::mem::transmute::<u8, Spring>(c as u8)) },
			_ => bail!("invalid char {c}"),
		}
	}
}

impl Debug for Spring {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_char(*self as u8 as char)
	}
}

#[cfg(test)]
mod tests {
	const EXAMPLE_1: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

	#[test]
	fn part_1_example_1() {
		assert_eq!(super::part_1(EXAMPLE_1).unwrap(), 21);
	}

	#[test]
	fn part_2_example_1() {
		assert_eq!(super::part_2(EXAMPLE_1).unwrap(), 525152);
	}

	#[test]
	fn difficult_example() {
		assert_eq!(super::part_2("?###???????? 3,2,1").unwrap(), 506250);
	}
}
