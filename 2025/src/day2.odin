package aoc

import "core:fmt"
import "core:os"
import "core:slice"
import "core:strconv"
import "core:strings"
import "core:testing"
import "core:math"

main :: proc() {
	input :: #load("./day2-input.txt", string)
	// input :: #load("./day2-example.txt", string)
	// input :: #load("./day2-example2.txt", string)

	fmt.printfln("Part 1: %d", day2part1(input))
	// fmt.printfln("Part 2: %d", day2part2(input))
}

day2part1 :: proc(input: string) -> int {
	string_ranges := strings.split(input, ",")

	sum := 0
	for s in string_ranges {
		if len(s) == 0 { continue }
		string_numbers := strings.split(s, "-")
		string_start := string_numbers[0]
		string_end := string_numbers[1]
		start, _ := strconv.parse_int(string_start) 
		end, _ := strconv.parse_int(string_end) 
		digit_half_count := len(string_start) / 2
		half_value, _ := strconv.parse_int(string_start[:digit_half_count])
		half_value_right, _ := strconv.parse_int(string_start[digit_half_count:])
		// fmt.printfln("From: %d, To: %d the invalid IDs are:", start, end)
		// fmt.printfln("=============================================")


		// If half_value right is larger than half_value then the full value from the
		// current half_value is unreachable
		if half_value < half_value_right {
			half_value += 1
		}
		// if the number is odd then get the minimum half count that would lead to an invalid id
		if len(string_start) % 2 == 1 {
			power_of_ten := pow_int(10, digit_half_count)
			half_value = power_of_ten
			digit_half_count += 1
		}
		// fmt.printfln("Current digit_half_count is %d and power of 10 is %d", digit_half_count, pow_int(10, digit_half_count))
		full_value := half_value * pow_int(10, digit_half_count) + half_value
		for full_value <= end {
			sum += full_value
			// fmt.printfln("%d", full_value)
			half_value += 1
			if half_value == pow_int(10, digit_half_count) { digit_half_count += 1 }
			full_value = half_value * pow_int(10, digit_half_count) + half_value
		}
		// fmt.printfln("=============================================")
	}
	return sum
}

pow_int :: proc(x, y: int) -> (int) {
	if y < 0 { return 1 }
	res := 1
    for _ in 0..<y {
        res *= x
    }
    return res
}

// day2part2 :: proc(input: string) -> int {
//
// }
