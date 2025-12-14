package aoc

import "core:fmt"
import "core:strings"
import "core:testing"
import "core:strconv"
import "core:os"
import "core:slice"

main :: proc() {
    input :: #load("./day1-input.txt", string)
    // input :: #load("./day1-example.txt", string)
    // input :: #load("./day1-example2.txt", string)

    // fmt.printfln("Part 1: %d", day1part1(input))
    fmt.printfln("Part 2: %d", day1part2(input))
}

day1part1 :: proc(input: string) -> int {
    rotations := strings.split(input, "\n")
	curr_val := 50

	zero_pointing_count := 0
    for i in 0..<len(rotations) {
		s := rotations[i]
		if len(s) == 0 {
			continue
		}
		dir := s[0]
		val, ok := strconv.parse_int(s[1:])
		if !ok {
			os.exit(1)
		}
		sign := dir == 'L' ? -1 : 1
		curr_val = (curr_val + (val * sign) + 100) % 100 
		if curr_val == 0 {
			zero_pointing_count += 1
		}
	}
	return zero_pointing_count
}

day1part2 :: proc(input: string) -> int {
    rotations := strings.split(input, "\n")
    curr_val := 50 // The dial starts at 50.

	cross_count := 0
    
    for s in rotations {
        if len(s) < 2 { continue }
        dir := s[0]
        val_str := s[1:]
        
        val, ok := strconv.parse_int(val_str)
        if !ok { os.exit(1) }
        
        rotation_sign := dir == 'L' ? -1 : 1
		first_cross_clicks: int = 0
		first_cross_clicks = rotation_sign == 1 ? 100 - curr_val : curr_val
		if curr_val == 0 { first_cross_clicks = 100 }

		if val >= first_cross_clicks {
			cross_count += 1
			remaining_clicks := val - first_cross_clicks
			cross_count += remaining_clicks / 100
		}
		curr_val = curr_val + (val * rotation_sign)
		curr_val = (curr_val % 100 + 100) % 100
	}

	return cross_count
}

@(test)
test_part_1 :: proc(t: ^testing.T) {
    sample :: #load("./day1-example.txt", string)
    testing.expect_value(t, day1part1(sample), 7)
}
