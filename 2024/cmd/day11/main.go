package main

import (
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

func main() {
	nums := parseInput()
	part2(nums)
}

func parseInput() []int {
	content, err := os.ReadFile("input/day11.txt")
	// content, err := os.ReadFile("input/test.txt")
    if err != nil { panic(err) }
	trimmed := strings.TrimRight(string(content), "\n")
    stringNums := strings.Split(trimmed, " ")
	nums := make([]int, len(stringNums))
	for i, str := range stringNums {
		nums[i], _ = strconv.Atoi(str)
	}
	return nums
}
// Starting with a string array of the original numbers for each blink:
// - If the num is zero make it one
// - If the len of numString is odd multiply by 2024
// - If instead it is even split the string down the middle, overriding the current
//      string with the left half, and adding a new string to the right of it with the right half

func part1(stringNums []string) {
	for j := 0; j < 25; j++ {
		for i := 0; i < len(stringNums); i++ {
			str := stringNums[i]
			num, _ := strconv.Atoi(str)
			if num == 0 {
				stringNums[i] = "1"
			} else if len(str) % 2 == 1 {
				stringNums[i] = strconv.Itoa(2024 * num)
			} else {
				middle := len(str) / 2
				stringNums[i] = str[:middle]
				secondHalf :=str[middle:]
				if len(secondHalf) > 1 { secondHalf = strings.TrimLeft(secondHalf, "0") }
				stringNums = slices.Insert(stringNums, i + 1, secondHalf)
				i++
			}
		}
	}
	fmt.Println(len(stringNums))
}
func part2(nums []int) {
	sum := 0
	memo := make(map[[2]int]int)
	for _, num := range nums {
		sum += howMany(&memo, num, 75)
	}
	fmt.Println(sum)
}

func howMany(memo *map[[2]int]int, val int, blinks int) int {
	if blinks == 0 {
		return 1
	} else {
		if _, ok := (*memo)[[2]int {val, blinks}]; ok {
			return (*memo)[[2]int {val, blinks}]
		}
		if val == 0 {
			(*memo)[[2]int {val, blinks}] = howMany(memo, 1, blinks-1)
			return (*memo)[[2]int {val, blinks}]
		} else if getDigitCount(val) % 2 == 0 {
			firstHalf, secondHalf := splitInHalf(val)
			(*memo)[[2]int {val, blinks}] = howMany(memo, firstHalf, blinks-1) + howMany(memo, secondHalf, blinks-1)
			return (*memo)[[2]int {val, blinks}]
		} else {
			(*memo)[[2]int {val, blinks}] = howMany(memo, val * 2024, blinks-1)
			return (*memo)[[2]int {val, blinks}]
		}
	}
}

func getDigitCount(val int) int {
	str := strconv.Itoa(val)
	return len(str)
}

func splitInHalf(val int) (int, int) {
	str := strconv.Itoa(val)
	middle := len(str) / 2
	firstStr, secondStr := str[:middle], str[middle:]
	firstInt, _ := strconv.Atoi(firstStr)
	secondInt, _ := strconv.Atoi(secondStr)
	return firstInt, secondInt
}
