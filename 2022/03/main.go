package main

import (
	"bufio"
	"log"
	"os"
)

func main() {
	f, err := os.Open("./2022/03/input.txt")

	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)

	// getItem := func(first, second string) int {
	// 	for _, n := range first {
	// 		for _, m := range second {
	// 			if n == m {
	// 				return int(n)
	// 			}
	// 		}
	// 	}
	// 	return 0
	// }

	getItem2 := func(lines [3]string) int {
		for _, n := range lines[0] {
			for _, m := range lines[1] {
				for _, k := range lines[2] {
					if n == m && m == k {
						return int(n)
					}
				}
			}
		}
		return 0
	}

	println('a'-96, 'z'-96, 'A'-38, 'Z')
	sum := 0
	var lines [3]string
	lineNo := 0
	for scanner.Scan() {
		line := scanner.Text()
		lines[lineNo] = line
		lineNo++
		if lineNo == 3 {
			lineNo = 0
			item := getItem2(lines)
			if item <= 90 {
				sum += item - 38
			} else {
				sum += item - 96
			}
		}

		// item := getItem(line[:len(line)/2], line[len(line)/2:])
		// if item <= 90 {
		// 	sum += item - 38
		// } else {
		// 	sum += item - 96
		// }
	}
	println(sum)
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
