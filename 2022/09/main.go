package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	f, err := os.Open("./2022/09/input.txt")

	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)

	type Point struct {
		x int64
		y int64
	}
	var rope [10]Point
	head := &rope[0]
	tail := &rope[len(rope)-1]

	positions := map[Point]bool{}
	fmt.Println(positions)
	for scanner.Scan() {
		line := scanner.Text()
		direction := line[0]
		steps, _ := strconv.ParseInt(line[2:], 10, 64)

		for i := int64(0); i < steps; i++ {
			switch direction {
			case 'R':
				head.x++
			case 'U':
				head.y++
			case 'D':
				head.y--
			case 'L':
				head.x--
			}
			for j := 1; j < len(rope); j++ {
				before := &rope[j-1]
				current := &rope[j]
				if !((current.x == before.x || current.x+1 == before.x || current.x-1 == before.x) &&
					(current.y == before.y || current.y+1 == before.y || current.y-1 == before.y)) {
					if current.y > before.y {
						current.y--
					} else if current.y < before.y {
						current.y++
					}
					if current.x > before.x {
						current.x--
					} else if current.x < before.x {
						current.x++
					}
				}
			}

			positions[*tail] = true

		}
		// fmt.Println(rope)

	}
	fmt.Println(len(positions))

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
