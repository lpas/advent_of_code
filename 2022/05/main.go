package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
)

func main() {
	f, err := os.Open("./2022/05/input.txt")

	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)

	count := 0
	var boxesRead []string

	mode := "readboxes"
	var columns int64
	var boxes [][]string
	for scanner.Scan() {
		line := scanner.Text()

		if mode == "empty" {
			mode = "commands"
			var c int64

			boxes = make([][]string, columns)
			for i := len(boxesRead) - 1; i >= 0; i-- {
				cbl := boxesRead[i]
				lcbl := int64(len(cbl))
				for c = 0; c < columns; c++ {
					pos := c*4 + 1
					if lcbl > pos && cbl[pos] != ' ' {
						boxes[c] = append(boxes[c], string(cbl[pos]))
					}
					// fmt.Println(cbl, pos, lcbl)
				}
			}

			continue
		}

		if mode == "readboxes" {
			if line[1] != '1' {
				boxesRead = append(boxesRead, line)
			} else {
				columns, _ = strconv.ParseInt(string(line[len(line)-2]), 10, 64)
				mode = "empty"
			}
			continue
		}

		r, _ := regexp.Compile("move ([0-9]*) from ([0-9]*) to ([0-9]*)")
		match := r.FindStringSubmatch(line)
		// fmt.Println(line, match)

		num, _ := strconv.ParseInt(match[1], 10, 64)
		from, _ := strconv.ParseInt(match[2], 10, 64)
		to, _ := strconv.ParseInt(match[3], 10, 64)
		var x []string
		// fmt.Println(line, num)
		// fmt.Println(boxes)
		// part1
		// for i := 0; i < int(num); i++ {
		// 	// fmt.Println("h1", boxes[from-1], boxes[to-1])
		// 	x, boxes[from-1] = boxes[from-1][len(boxes[from-1])-1], boxes[from-1][:len(boxes[from-1])-1]
		// 	boxes[to-1] = append(boxes[to-1], x)
		// }
		// fmt.Println(boxes)

		// part2
		// fmt.Println(num, from, to, x)
		// fmt.Println(boxes)
		l := len(boxes[from-1])
		x, boxes[from-1] = boxes[from-1][l-int(num):l], boxes[from-1][:l-int(num)]
		boxes[to-1] = append(boxes[to-1], x...)
		// fmt.Println(boxes)

	}
	// fmt.Println(boxes, columns)
	fmt.Println(count, boxes)
	result := ""
	for _, item := range boxes {
		top := item[len(item)-1]
		result = result + top
	}
	fmt.Println(result)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
