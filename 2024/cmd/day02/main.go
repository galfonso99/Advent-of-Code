package main

import (
    "fmt"
    "os"
    "strings"
	"strconv"
)

func main() {
	// content, err := os.ReadFile("input/test.txt")
	content, err := os.ReadFile("input/day02.txt")
	// content, err := os.ReadFile("input/test1.txt")
    if err != nil { panic(err) }
	trimmed := strings.TrimRight(string(content), "\n")
    lines := strings.Split(trimmed, "\n")
	fmt.Println(len(lines))

	nums := make([][]int, len(lines))
    for i, line := range lines {
		values := strings.Split(line, " ")
		if len(line) == 0 { continue }
		nums[i] = make([]int, len(values))
		for j, val := range values {
			num, _ := strconv.Atoi(val)
			nums[i][j] = num
		}
    }
	res := part2(nums)
	fmt.Println(res)
}

func part1(nums [][]int) int {
	count := 0
	for _, arr := range nums {
		if isSafe(arr) {
			count += 1
		}
	}
	return count
}

func isSafe(arr []int) bool {
	if len(arr) == 1 { return true }
	var dir int
	if arr[1] - arr[0] >= 0 {
		dir = 1
	} else {
		dir = -1
	}
	for i := 1; i < len(arr); i++ {
		diff := arr[i] - arr[i-1]
		if diff * dir <= 0 { return false }
		if diff < 0 { diff = -diff }
		if diff < 1 || diff > 3 {
			return false
		}
	}
	return true
}

func part2(nums [][]int) int {
	count := 0
	for _, arr := range nums {
		printArr := make([]int, len(arr))
		_ = copy(printArr, arr)
		for i := range arr {
		copyArr := make([]int, len(arr))
		_ = copy(copyArr, arr)
			if isSafe(append(copyArr[:i], copyArr[i+1:]...)) {
				fmt.Println(printArr)
				count += 1
				break
			}
		}
	}
	// safeCount := 0
	// unsafeCount := 0
	// for _, arr := range nums {
	// 	if isSafeWithDampener(arr) {
	// 		// fmt.Printf("Line was safe as index: %d\n", i+1)
	// 		fmt.Println(arr)
	// 		count += 1
	// 	} else {
	// 		// fmt.Println(arr)
	// 	}
	// }
	return count
}

func isSafeWithDampener(arr []int) bool {
	if len(arr) == 1 { return true }
	prev_dir := 0
	dir := 0
	fault_count := 0
	for i := 1; i < len(arr); i++ {
		diff := arr[i] - arr[i-1]
		if diff * dir < 0 { 
			if fault_count > 0 {
				// fmt.Println(arr)
				return false
			} else {
				fault_count += 1
				dir = prev_dir
				arr[i] = arr[i-1]
				continue
			}
		}
		original_val := diff
		if diff < 0 { diff = -diff }
		if diff < 1 || diff > 3 {
			if fault_count > 0 {
				return false
			} else {
				fault_count += 1
				dir = prev_dir
				arr[i] = arr[i-1]
				continue
			}
		}
		prev_dir = dir
		if original_val > 0 {
			dir = 1
		} else {
			dir = -1
		}
	}
	return true
}
// 5 3 6 8 10
  // -1 0 1 1 1
  //  0 0 0 1 1
// 3 5 2 4 6 8
// going up 
// going down ignore
// going up 
// going up

// For part 1
// Make a slice of ints based on the line levels
// Loop over the slice and test if the line is 'safe'
// A line becomes unsafe when it increases and decreases at the same time, or if
//    the difference between adjacent values is less than 1 or bigger than 3
// Keep a counter of the lines which are safe
// Return the count
