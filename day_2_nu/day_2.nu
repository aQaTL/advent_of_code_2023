#!/usr/bin/env nu

let lines = $env.FILE_PWD | path join .. day_2 input.txt | open | lines

let games = (
	$lines | 
	split column ": " game_id bags 
	| each {
		{
			game_id: ($in.game_id | parse "Game {game_id}" | get game_id.0 | into int),
			bags: (
				$in.bags | 
				split row "; " | 
				each { (
					$in | 
					split row ", " |
					parse "{count} {color}" 
				) }
			)
		}
	}
)

print $"Part 1: (part_1 $games)"
print $"Part 2: (part_2 $games)"

def part_1 [games] {
	(
		$games | 
		filter { |game|
			$game.bags | 
			all { |bag|
				let red_count = $bag | find color == "red" | get -i count.0 | default "0" | into int
				let green_count = $bag | find color == "green" | get -i count.0 | default "0" | into int
				let blue_count = $bag | find color == "blue" | get -i count.0 | default "0" | into int
				$red_count <= 12 and $green_count <= 13 and $blue_count <= 14
			}
		} | 
		get game_id | 
		math sum
	)
}

def part_2 [games] {
	(
		$games | 
		each { |game|
			$game.bags | 
			each { |bag|
				let red_count = $bag | find color == "red" | get -i count.0 | default "0" | into int
				let green_count = $bag | find color == "green" | get -i count.0 | default "0" | into int
				let blue_count = $bag | find color == "blue" | get -i count.0 | default "0" | into int
				{red: $red_count, green: $green_count, blue: $blue_count}
			} | 
			math max |
			transpose color count |
			get count |
			math product
		} | 
		math sum
	)
}
