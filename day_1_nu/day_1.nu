#!/usr/bin/env nu

let lines = $env.FILE_PWD | path join .. day_1 input.txt | open | lines

print $"Day 1: (day_1 $lines)"
print $"Day 2: (day_2_ver2 $lines)"

def day_1 [input: list<string>] {
	(
		$input | 
		each { 
			$in | 
			parse --regex '(?<digit>\d)' | 
			get digit | 
			into int | 
			do {
				(($in | first) * 10) + ($in | last)
			}
		} | 
		math sum
	)
}

def day_2_ver2 [input: list<string>] {
	let digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
	(
		$input | 
		each { |line|
			let final_digits = (
				$line | 
				split chars | 
				enumerate | 
				each { |it|
					let x = $it.item
					let idx = $it.index

					if $x =~ '\d' {
						return $x
					} 

					let slice = $line | str substring $idx..

					let digit_idx = (
						$digits |  
						enumerate | 
						where { |it| 
							($slice | str starts-with $it.item) 
						} |
						get index
					)
					if (not ($digit_idx | is-empty)) {
						let final_digit = ($digit_idx | get 0 | into int) + 1
						return $final_digit | into string
					}
				}
			)

			let first_d = $final_digits | first
			let last_d = $final_digits | last
			$"($first_d)($last_d)" | into int
		} | 
		math sum
	)
}
