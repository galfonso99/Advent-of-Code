package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	nums := parseInput()
	part1(nums)
}

func parseInput() []int {
	content, err := os.ReadFile("input/day09.txt")
	// content, err := os.ReadFile("input/test.txt")
	if err != nil { panic(err) }
	trimmed := strings.TrimRight(string(content), "\n")
	split := strings.Split(trimmed, "")
	fmt.Println(len(split))
	nums := make([]int, len(split))
	for i, digit := range split {
		nums[i], _ = strconv.Atoi(digit)
	}
	return nums
}
// Parse the input into an array of int
// Create a memory map (array) with the id in the corresponding index and 0 for empty space
// Simultaneously create an array of the starting index of empty spaces
// After the memory map is completed, loop backwards swapping the last value with the
// currently first empty spot.
// Once you run out of empty spots in the current section look for the first index of the next
// section of empties
// THENNNN, Calculate the sum of the multiplication of the index with its corresponding value

func part1(nums []int) {
	i := 0
	sum := 0
	numSum := 0
	for i := 0; i < len(nums); i++ {
		numSum += nums[i]
	}
	memoryMap := make([]int, numSum)
	emptyIndices := make([]int, 0, 5)
	for j := 0; j < len(nums); j++ {
		if j % 2 == 0 {
			fillRange(memoryMap, i, i + nums[j], j / 2)
			i += nums[j]
		} else if j % 2 == 1 && nums[j] != 0 {
			for x := i; x < i + nums[j]; x++ {
				emptyIndices = append(emptyIndices, x) 
			}
			i += nums[j]
		}
	}
	x := 0
	for i := len(memoryMap) - 1; i >= 0; i-- {
		if memoryMap[i] == 0 { 
			continue 
		}
		if emptyIndices[x] > i { 
			break 
		}
		memoryMap[emptyIndices[x]] = memoryMap[i]
		memoryMap[i] = 0
		// fmt.Println(memoryMap)
		x++
	}
	// fmt.Println(memoryMap)
	for i, num := range memoryMap {
		if sum > 0 && num == 0 { break }
		sum += i * num
	}
	fmt.Println(sum)
}
		
func fillRange(slice []int, start int, end int, val int) {
	for i := start; i < end; i++ {
		slice[i] = val
	}
}

func part2(nums []int) {
	
}

