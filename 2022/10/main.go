package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	f, err := os.Open("./2022/10/input.txt")

	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)
	x := int64(1)
	cycleCounter := int64(0)
	sum := int64(0)
	cycle := func() {
		pos := cycleCounter % 40

		if pos == x || pos+1 == x || pos-1 == x {
			fmt.Print("#")
		} else {
			fmt.Print(".")
		}
		if pos == 39 {
			fmt.Println()
		} // FGCUZREC

		cycleCounter++

		if (cycleCounter-20)%40 == 0 {
			sum += cycleCounter * x
		}

		// fmt.Println(cycleCounter, x)
	}

	for scanner.Scan() {
		line := scanner.Text()
		command := line[:4]

		// fmt.Println(command)

		switch command {
		case "noop":
			cycle()
		case "addx":
			cycle()
			cycle()
			v, _ := strconv.ParseInt(line[5:], 10, 64)
			x += v
		}
	}
	fmt.Println(sum)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
