package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func main() {
	f, err := os.Open("./2022/08/input.txt")

	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)
	var trees [][]int
	for scanner.Scan() {
		line := scanner.Text()
		var new []int
		for _, v := range line {
			new = append(new, int(v-'0'))
		}
		trees = append(trees, new)
		// fmt.Println(line)
	}

	getCol := func(col int) (column []int) {
		column = make([]int, len(trees))
		for i, row := range trees {
			column[i] = row[col]
		}
		return
	}
	tallerOrSame := func(num int, slice []int) bool {
		for _, v := range slice {
			if v >= num {
				return true
			}
		}
		return false
	}

	count := 0
	for i, row := range trees {
		if i == 0 || i == len(trees)-1 {
			count += len(row)
			continue
		}
		for j, current := range row {
			if j == 0 || j == len(row)-1 {
				count++
				continue
			}
			col := getCol(j)
			if !(tallerOrSame(current, row[:j]) &&
				tallerOrSame(current, row[j+1:]) &&
				tallerOrSame(current, col[:i]) &&
				tallerOrSame(current, col[i+1:])) {
				count++
			}
		}

	}
	fmt.Println(count)

	// step2
	bestSienceScore := 0
	for i, row := range trees {
		for j, current := range row {
			left := 0
			for n := j - 1; n >= 0; n-- {
				left++
				if row[n] >= current {
					break
				}
			}
			right := 0
			for n := j + 1; n < len(row); n++ {
				right++
				if row[n] >= current {
					break
				}
			}
			top := 0
			for n := i - 1; n >= 0; n-- {
				top++
				if trees[n][j] >= current {
					break
				}
			}
			bottom := 0
			for n := i + 1; n < len(trees); n++ {
				bottom++
				if trees[n][j] >= current {
					break
				}
			}
			currentSienceScore := left * right * top * bottom
			if currentSienceScore > bestSienceScore {
				bestSienceScore = currentSienceScore
			}
			// fmt.Println(i, j, current, left, right, top, bottom, currentSienceScore)

		}
	}
	fmt.Println(bestSienceScore)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
