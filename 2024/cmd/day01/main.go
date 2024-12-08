package main

import (
    "fmt"
    "os"
    "strings"
	"sort"
	"strconv"
)

func main() {
	// content, err := os.ReadFile("input/test.txt")
	content, err := os.ReadFile("input/day01.txt")
    if err != nil { panic(err) }
    lines := strings.Split(string(content), "\n")
	first := make([]int, len(lines))
	second := make([]int, len(lines))
    for i, line := range lines {
		nums := strings.Split(line, "   ")
		if len(line) == 0 { continue }
		first[i], err = strconv.Atoi(nums[0])
		second[i], err = strconv.Atoi(nums[1])
    }
	res := part2(first, second)
	fmt.Println(res)
}

func part1(arr1 []int, arr2 []int) int {
	sort.Ints(arr1)
	sort.Ints(arr2)
	total := 0
	for i := range arr1 {
		dist := arr2[i] - arr1[i]
		if dist < 0 { dist = -dist }
		total += dist
	}
	return total
}

func part2(arr1 []int, arr2 []int) int {
	Map := make(map[int]int)
	similarity := 0
	for i := range arr2 {
		if val, ok := Map[arr2[i]]; ok {
			Map[arr2[i]] = val + 1
		} else {
			Map[arr2[i]] = 1
		}
	}
	for i := range arr1 {
		sim := arr1[i] * Map[arr1[i]] 
		if sim > 0 {fmt.Printf("%d \n", sim)}
		similarity += sim
	}
	return similarity
}

/* Some snippets of code of interest  */

// sort.Slice(arr, func(i, j int) bool {
//        if arr[i][0] != arr[j][0] {
//            return arr[i][0] < arr[j][0]
//        }
//        return arr[i][1] < arr[j][1]
//    })

// for key, value := range Map {
// 	fmt.Printf("%d %d \n", key, value)
// }
