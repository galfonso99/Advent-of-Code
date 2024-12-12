package main

import (
	"fmt"
	"os"
	"strconv"
	"unicode"
)

func main() {
	// content, err := os.ReadFile("input/day03.txt")
	content, err := os.ReadFile("input/test.txt")
    if err != nil { panic(err) }
	part2(string(content))
}

func part1(str string) {
	sum := 0
	a := 0
	b := 0
	sample := "mul(a,b)"
	sequence := 0
	i := 0
	for i < len(str) {
		if sequence == len(sample) - 1 && str[i] == sample[len(sample) - 1] {
			sum += a * b
			sequence = 0
		}
		if sequence == 4 || sequence == 6 {
			strVal := ""
			j := 0
			if !unicode.IsDigit(rune(str[i])) {
				sequence = 0
				i++
				continue
			}
			for unicode.IsDigit(rune(str[i])) && j < 3 {
				strVal += string(str[i])
				j++
				i++
			}
			val, _ := strconv.Atoi(strVal)
			if sequence == 4 {
				a = val
			} else {
				b = val
			}
			sequence++
			continue 
		}
		if str[i] == sample[sequence] {
			sequence++
		} else {
			sequence = 0
		}
		i++
	}
	fmt.Println(sum)
}

func part2(str string) {
	samples := []string {"mul(a,b)", "do()", "don't()"}
	sequences := []int {0, 0, 0}
	sum := 0
	greenLight := true
	a := 0
	b := 0
	for i := 0; i < len(str); {
		for j := 0; j < 3; j++ {
			if sequences[j] == len(samples[j]) - 1 && str[i] == samples[j][len(samples[j]) - 1] {
				sequences[j] = 0
				// fmt.Printf("A: %d, B: %d\n", a, b)
				if greenLight {
					fmt.Printf("greenLight is active\n")
				} else {
					fmt.Printf("greenLight is NOT active\n")
				}
				if j == 0 && greenLight { sum += a * b }
				if j == 1 { greenLight = true }
				if j == 2 { greenLight = false }
			}
		}
		if sequences[0] == 4 || sequences[0] == 6 {
			strVal := ""
			j := 0
			if !unicode.IsDigit(rune(str[i])) {
				sequences[0] = 0
				i++
				continue
			}
			for unicode.IsDigit(rune(str[i])) && j < 3 {
				strVal += string(str[i])
				j++
				i++
			}
			val, _ := strconv.Atoi(strVal)
			if sequences[0] == 4 {
				a = val
			} else {
				b = val
			}
			sequences[0]++
			continue 
		}
		for j := 0; j < 3; j++ {
			if str[i] == samples[j][sequences[j]] {
				sequences[j]++
			} else {
				sequences[j] = 0
			}
		}
		fmt.Printf("Current index : %d, current char %s, sequence: %d\n", i, string(str[i]), sequences[0])
		i++
	}
	fmt.Println(sum)
}
// func part2(nums [][]int) {
//
// }

