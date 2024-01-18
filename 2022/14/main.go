package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func getInt(val string) int {
	value, _ := strconv.ParseInt(val, 10, 64)
	return int(value)
}

func main() {
	f, err := os.Open("./2022/14/input.txt")

	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)
	startX, startY := 500, 0
	minX, maxX, minY, maxY := startX, startX, startY, startY

	var grid [1000][1000]rune
	for x, col := range grid {
		for y := range col {
			grid[x][y] = '.'
		}
	}

	for scanner.Scan() {
		line := scanner.Text()
		split := strings.Fields(line)
		currX, currY := -1, -1
		for index, item := range split {
			if item == "->" {
				continue
			}
			xy := strings.Split(item, ",")
			x, y := getInt(xy[0]), getInt(xy[1])
			if x < minX {
				minX = x
			}
			if x > maxX {
				maxX = x
			}
			if y < minY {
				minY = y
			}
			if y > maxY {
				maxY = y
			}

			if index == 0 {
				currX = x
				currY = y
				continue
			}
			for {
				grid[currX][currY] = '#'
				if currX == x && currY == y {
					break
				}

				if currX != x {
					if currX < x {
						currX++
					} else {
						currX--
					}
				}
				if currY != y {
					if currY < y {
						currY++
					} else {
						currY--
					}
				}
			}

		}
	}

	for x := range grid {
		grid[x][maxY+2] = '#'
	}

	printGrid := func() {
		return
		for y := minY; y <= maxY; y++ {
			for x := minX; x <= maxX; x++ {
				fmt.Print(string(grid[x][y]))
			}
			fmt.Print("\n")
		}
	}
	// printGrid()

	rounds := 0
out:
	for {

		posX, posY := startX, startY
		for {

			// step1
			// if posX > maxX || posX < minX || posY > maxY || posY < minY {
			// 	break out
			// }

			if grid[posX][posY+1] == '.' {
				posY++
				continue
			}
			if grid[posX-1][posY+1] == '.' {
				posX--
				posY++
				continue
			}
			if grid[posX+1][posY+1] == '.' {
				posX++
				posY++
				continue
			}

			grid[posX][posY] = 'o'
			rounds++
			if posX == startX && posY == startY {
				break out
			}
			break
		}
	}

	fmt.Println(rounds)

	printGrid()

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
