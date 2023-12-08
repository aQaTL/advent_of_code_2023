use anyhow::bail;
use itertools::Itertools;
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_8/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> anyhow::Result<i64> {
	let (steps, nodes) = parse_input(input)?;

	let mut current = "AAA";
	let mut step_count = 0;
	for step in steps.iter().copied().cycle() {
		if current == "ZZZ" {
			break;
		}
		current = nodes[current][step];
		step_count += 1;
	}

	Ok(step_count)
}

fn part_2(input: &str) -> anyhow::Result<i64> {
	let (steps, nodes) = parse_input(input)?;

	let mut currents: Vec<&str> = nodes
		.keys()
		.filter(|x| x.ends_with("A"))
		.map(|x| *x)
		.collect();
	let mut currents_step_counts = Vec::new();
	let currents_len = currents.len();
	for (idx, step) in steps.iter().copied().cycle().enumerate() {
		if currents_step_counts.len() == currents_len {
			break;
		}
		currents.retain_mut(|current| {
			*current = nodes[current][step];
			if current.ends_with("Z") {
				currents_step_counts.push(idx as i64 + 1);
				return false;
			}
			true
		});
	}

	Ok(currents_step_counts.into_iter().fold(1, lcm))
}

fn lcm(a: i64, b: i64) -> i64 {
	(a * b).abs() / gcd(a, b)
}

fn gcd(a: i64, b: i64) -> i64 {
	if b == 0 {
		a
	} else {
		gcd(b, a.rem_euclid(b))
	}
}

fn parse_input(input: &str) -> anyhow::Result<(Vec<usize>, HashMap<&str, [&str; 2]>)> {
	let mut lines = input.lines();
	let steps: Vec<usize> = lines
		.next()
		.unwrap()
		.bytes()
		.map(|b| match b {
			b'L' => Ok(0),
			b'R' => Ok(1),
			_ => bail!("invalid step {}", b as char),
		})
		.try_collect()?;

	let _ = lines.next().unwrap();

	let mut nodes = HashMap::<&str, [&str; 2]>::new();
	for node in lines {
		let (node, next_nodes) = node.split(" = (").next_tuple().unwrap();
		let (next_node_a, next_node_b) = next_nodes.split(", ").next_tuple().unwrap();
		nodes.insert(node, [next_node_a, &next_node_b[0..3]]);
	}

	Ok((steps, nodes))
}

#[cfg(test)]
mod tests {
	const EXAMPLE_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

	#[test]
	fn part_1_example_1() {
		assert_eq!(super::part_1(EXAMPLE_1).unwrap(), 2);
	}

	const EXAMPLE_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

	#[test]
	fn part_1_example_2() {
		assert_eq!(super::part_1(EXAMPLE_2).unwrap(), 6);
	}

	const EXAMPLE_3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

	#[test]
	fn part_2_example_3() {
		assert_eq!(super::part_2(EXAMPLE_3).unwrap(), 6);
	}
}
