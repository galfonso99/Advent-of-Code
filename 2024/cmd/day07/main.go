package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	testVals, nums := parseInput()
	part2(testVals, nums)
}

func parseInput() ([]int, [][]int) {
	content, err := os.ReadFile("input/day07.txt")
	// content, err := os.ReadFile("input/test.txt")
    if err != nil { panic(err) }
	trimmed := strings.TrimRight(string(content), "\n")
	lines := strings.Split(trimmed, "\n")
	testVals := make([]int, len(lines))
	nums := make([][]int, len(lines))
	for i, line := range lines {
		equationSplit := strings.Split(line, ":")
		testVals[i], _ = strconv.Atoi(equationSplit[0])
		equationSplit[1] = strings.TrimLeft(equationSplit[1], " ")
		numsStr := equationSplit[1]
		numsSplit := strings.Split(numsStr, " ")
		nums[i] = make([]int, len(numsSplit))
		for j, numStr := range numsSplit {
			nums[i][j], _ = strconv.Atoi(numStr)
		}
	}
	return testVals, nums
}

func part1(testVals []int, nums [][]int) {
	total := 0
	for i := 0; i < len(nums); i++ {
		var isTrue bool 
		isTrueEquation(nums[i], testVals[i], nums[i][0], 1, &isTrue)
		if isTrue {
			total += testVals[i]
		}
	}
	fmt.Println(total)
}

func isTrueEquation(nums []int, testVal int, val int, ind int, isTrue *bool) {
	if ind == len(nums) && val == testVal {
		*isTrue = true
		return
	}
	if ind == len(nums) || *isTrue { return }
	for i := 0; i < 2; i++ {
		if i % 2 == 0 {
			isTrueEquation(nums, testVal, val + nums[ind], ind + 1, isTrue)
		} else {
			isTrueEquation(nums, testVal, val * nums[ind], ind + 1, isTrue)
		}
	}
}

func part2(testVals []int, nums [][]int) {
	total := 0
	for i := 0; i < len(nums); i++ {
		var isTrue bool 
		isTrueEquation2(nums[i], testVals[i], nums[i][0], 1, &isTrue)
		if isTrue {
			total += testVals[i]
		}
	}
	fmt.Println(total)
}

func isTrueEquation2(nums []int, testVal int, val int, ind int, isTrue *bool) {
	if ind == len(nums) && val == testVal {
		*isTrue = true
		return
	}
	if ind == len(nums) || *isTrue { return }
	for i := 0; i < 3; i++ {
		if i == 0 {
			isTrueEquation2(nums, testVal, val + nums[ind], ind + 1, isTrue)
		} else if i == 1 {
			isTrueEquation2(nums, testVal, val * nums[ind], ind + 1, isTrue)
		} else {
			isTrueEquation2(nums, testVal, concat(val, nums[ind]), ind + 1, isTrue)
		}
	}
}

func concat(num1 int, num2 int) int {
	str1 := strconv.Itoa(num1)
	str2 := strconv.Itoa(num2)
	resultStr := str1 + str2
	result, _ := strconv.Atoi(resultStr)
	return result
}
