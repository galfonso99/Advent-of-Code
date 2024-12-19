package main

import (
	"fmt"
	"os"
	"strings"
	"strconv"
)

func main() {
	ruleMap, updateList := parseInput()
	part2(ruleMap, updateList)
}

func parseInput() ([100][]int, [][]int) {
	content, err := os.ReadFile("input/day05.txt")
	// content, err := os.ReadFile("input/test.txt")
    if err != nil { panic(err) }
	trimmed := strings.TrimRight(string(content), "\n")
	sections := strings.Split(trimmed, "\n\n")
	var ruleMap [100][]int
	rules := strings.Split(sections[0], "\n")
	for _, rule := range rules {
		numStrs := strings.Split(rule, "|")
		num1, _ := strconv.Atoi(numStrs[0])
		num2, _ := strconv.Atoi(numStrs[1])
		if ruleMap[num1] == nil {
			ruleMap[num1] = make([]int, 0, 5)
		}
		ruleMap[num1] = append(ruleMap[num1], num2)
	}
	updates := strings.Split(sections[1], "\n")
	updateList := make([][]int, len(updates))
	for i, update := range updates {
		numStrs := strings.Split(update, ",")
		updateList[i] = make([]int, len(numStrs))
		for j, numStr := range numStrs {
			num, _ := strconv.Atoi(numStr)
			updateList[i][j] = num
		}
	}
	return ruleMap, updateList
}

func part1(ruleMap [100][]int, updateList [][]int) {
	sum := 0
	outer: for i, update := range updateList {
		var pageMap [100]int
		// Make a map using array from value to index
		for j, page := range update {
			pageMap[page] = j+1
		}
		// For each page in current update get all the secondPages that correspond to that
		// page, and if both that first page and second page exist in current update then
		// make sure that the index of first page is less than second and if so add the 
		// middle value of the current update to totalSum
		for _, page := range update {
			if ruleMap[page] == nil { continue }
			for _, secondPage := range ruleMap[page] {
				if pageMap[secondPage] == 0 { continue }
				if pageMap[page] > pageMap[secondPage] {
					continue outer
				}
			}
		}
		mid := len(updateList[i]) / 2
		midVal := updateList[i][mid]
		sum += midVal
	}
	fmt.Println(sum)
}

func part2(ruleMap [100][]int, updateList [][]int) {
	sum := 0
	for i, update := range updateList {
		var pageMap [100]int
		incorrect := false
		for j, page := range update {
			pageMap[page] = j+1
		}
		for j := 0; j < len(update); j++ {
			page := update[j]
			if ruleMap[page] == nil { continue }
			for _, secondPage := range ruleMap[page] {
				if pageMap[secondPage] == 0 { continue }
				if pageMap[page] > pageMap[secondPage] {
					// Swapped the values and the indices
					update[pageMap[page] - 1], update[pageMap[secondPage] - 1] = update[pageMap[secondPage] - 1], update[pageMap[page] - 1]
					pageMap[page], pageMap[secondPage] = pageMap[secondPage], pageMap[page]
					// Change the index back to the swapped value that was behind
					j = pageMap[page] - 1 - 1   // Extra minus 1, accounting for the j++ of the for loop
					incorrect = true
					// Now just break because the swap messed everything up
					break
				}
			}
		}
		if incorrect {
			mid := len(updateList[i]) / 2
			midVal := update[mid]
			sum += midVal
		}
	}
	fmt.Println(sum)
}

// For the rules section create a hashmap int to []int using the first value as the key
// and an array of values that should appear after it as the "value"

// Then for the pages section, for each list of pages loop through the list and 
// put them in a hashmap with the key being the value and the value being the index base 1 because default is 0
// Then using this hashmap iterate through all the rules and if both number are found
// in the hashmap then make sure the first number has a lower index and if it does
// then this update is correct so grab the middle value and add it to the sum. If any
// of this is not true then the update is not correct so ignore it and move to the next 
// update

// 86,96,56,92,68
// 68,96,56,92,86
// 56,96,68,92,86
// 56,68,96,92,86
// 56,68,86,92,96
//
// 56|68
// 68|86
// 68|96
// 86|96
