package main

import (
	"container/list"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

func main() {
	stringNums := parseInput()
	part2(stringNums)
}

func parseInput() []string {
	// content, err := os.ReadFile("input/day11.txt")
	content, err := os.ReadFile("input/test.txt")
    if err != nil { panic(err) }
	trimmed := strings.TrimRight(string(content), "\n")
    stringNums := strings.Split(trimmed, " ")
	// nums := make([]int, len(stringNums))
	// for i, str := range stringNums {
	// 	nums[i], _ = strconv.Atoi(str)
	// }
	return stringNums
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
		// fmt.Println(stringNums)
	}
	fmt.Println(len(stringNums))
}
func part2(stringNums []string) {
	numsList := list.New()
	for _, str := range stringNums {
		numsList.PushBack(str)
	}
	for j := 0; j < 32; j++ {
		numStr := numsList.Front()
		for numStr != nil {
			num, _ := strconv.Atoi(numStr.Value.(string))
			if num == 0 {
				numStr.Value = "1"
			} else if len(numStr.Value.(string)) % 2 == 1 {
				numStr.Value = strconv.Itoa(2024 * num)
			} else {
				// fmt.Println(numStr.Value.(string))
				middle := len(numStr.Value.(string)) / 2
				secondHalf := numStr.Value.(string)[middle:]
				numStr.Value = numStr.Value.(string)[:middle]
				// fmt.Println("Printing halves")
				// fmt.Println(numStr.Value.(string)[:middle], secondHalf)
				if len(secondHalf) > 1 { secondHalf = strings.TrimLeft(secondHalf, "0") }
				if len(secondHalf) == 0 { secondHalf = "0" }
				numsList.InsertAfter(secondHalf, numStr)
				numStr = numStr.Next()
			}
			numStr = numStr.Next()
		}
		// for e := numsList.Front(); e != nil; e = e.Next() {
		// 	fmt.Printf("%s ", e.Value)
		// }
		// fmt.Println()
	}
	fmt.Println(numsList.Len())
}

// func replace(old *list.Element, new string) {
// 	if old.Next() != nil {
// 		next := old.Next()
// 	} else {
//
// 	}
// }
