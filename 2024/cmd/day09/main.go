package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	nums := parseInput()
	part2(nums)
}

func parseInput() []int {
	content, err := os.ReadFile("input/day09.txt")
	// content, err := os.ReadFile("input/test.txt")
	if err != nil { panic(err) }
	trimmed := strings.TrimRight(string(content), "\n")
	split := strings.Split(trimmed, "")
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

// Instead of keeping track of empty indices keep an array of 2 sized arrays with the starting index
// and the size of the space
// For each file see if there is a space where it could fit and put it there, if not then leave it
func part2(nums []int) {
	i := 0
	sum := 0
	numSum := 0
	for i := 0; i < len(nums); i++ {
		numSum += nums[i]
	}
	memoryMap := make([]int, numSum)
	emptyAreas := make([][2]int, 0, 5)
	fileGroups := make([][2]int, 0, 5)
	for j := 0; j < len(nums); j++ {
		if j % 2 == 0 {
			fillRange(memoryMap, i, i + nums[j], j / 2)
			fileGroups = append(fileGroups, [2]int {i, nums[j]} )
		} else if j % 2 == 1 && nums[j] != 0 {
			emptyAreas = append(emptyAreas, [2]int {i, nums[j]})
		}
		i += nums[j]
	}
	for i := len(fileGroups) - 1; i >= 0; i-- {
		areaIndex := findArea(fileGroups[i], emptyAreas)
		if areaIndex == -1 {      // No area was found
			continue
		}
		swapFilePosition(memoryMap, fileGroups[i], emptyAreas[areaIndex])
		emptyAreas[areaIndex][0] += fileGroups[i][1]
		emptyAreas[areaIndex][1] -= fileGroups[i][1]
	}
	// fmt.Println(memoryMap)
	for i, num := range memoryMap {
		sum += i * num
	}
	fmt.Println(sum)
}

func findArea(fileGroup [2]int, emptyAreas [][2]int) int {
	// Loop through all the emptyAreas from the start and check if the size is enough and
	// the index is before the file index
	for i, emptyArea := range emptyAreas {
		if emptyArea[1] >= fileGroup[1] && emptyArea[0] < fileGroup[0] {
			return i
		}
	}
	return -1
}

func swapFilePosition(memoryMap []int, fileGroup [2]int, emptyAreas [2]int) {
	for i := 0; i < fileGroup[1]; i++ {
		memoryMap[emptyAreas[0] + i] = memoryMap[fileGroup[0] + i]
		memoryMap[fileGroup[0] + i] = 0
	}
}

