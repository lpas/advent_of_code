package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func main() {
	f, err := os.Open("./2022/12/input.txt")

	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)

	type Pos struct {
		x int
		y int
	}

	var grid [][]int
	var start Pos
	var end Pos
	y := 0
	for scanner.Scan() {
		line := scanner.Text()
		currentLine := make([]int, len(line))
		grid = append(grid, currentLine)
		for x, char := range line {
			if char == 'S' {
				start = Pos{x: x, y: y}
				currentLine[x] = 1
			} else if char == 'E' {
				end = Pos{x: x, y: y}
				currentLine[x] = 26
			} else {
				currentLine[x] = int(char) - 96
			}
		}
		y++

		fmt.Println(line)
	}

	maxX := len(grid[0])
	maxY := len(grid)
	neighbors := func(pos Pos) []Pos {
		height := grid[pos.y][pos.x]
		result := make([]Pos, 0)
		if pos.y > 0 && grid[pos.y-1][pos.x]+1 >= height {
			result = append(result, Pos{pos.x, pos.y - 1})
		}
		if pos.x+1 < maxX && grid[pos.y][pos.x+1]+1 >= height {
			result = append(result, Pos{pos.x + 1, pos.y})
		}
		if pos.y+1 < maxY && grid[pos.y+1][pos.x]+1 >= height {
			result = append(result, Pos{pos.x, pos.y + 1})
		}
		if pos.x > 0 && grid[pos.y][pos.x-1]+1 >= height {
			result = append(result, Pos{pos.x - 1, pos.y})
		}
		return result
	}

	frontier := make([]Pos, 0)
	frontier = append(frontier, end)
	cameFrom := make(map[Pos]Pos)
	cameFrom[end] = Pos{-1, -1}
	for len(frontier) > 0 {
		item := frontier[0]
		frontier = frontier[1:]
		// fmt.Println("visiting", item)
		for _, next := range neighbors(item) {
			_, ok := cameFrom[next]
			if !ok {
				frontier = append(frontier, next)
				cameFrom[next] = item
			}
		}
	}

	for y := 0; y < maxY; y++ {
		for x := 0; x < maxX; x++ {
			item, ok := cameFrom[Pos{x, y}]
			if !ok {
				fmt.Print("x")
			} else if item.x > x {
				fmt.Print("<")
			} else if item.y > y {
				fmt.Print("^")
			} else if item.x < x {
				fmt.Print(">")
			} else if item.y < y {
				fmt.Print("v")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Println()
	}

	fmt.Println(start, end)

	counter := 0
	minCounter := -1
	// positions := []Pos{start} // step 1
	positions := make([]Pos, 0)
	for y, line := range grid {
		for x, val := range line {
			if val == 1 {
				positions = append(positions, Pos{x, y})
			}
		}
	}

	for _, pos := range positions {
		counter = 0
		current := pos
		for current != end {
			counter++
			current, _ = cameFrom[current]
		}
		if minCounter == -1 || minCounter > counter {
			minCounter = counter
		}

	}
	fmt.Println(minCounter)

	// fmt.Println(grid, start, end, frontier, cameFrom)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
