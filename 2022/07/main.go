package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	f, err := os.Open("./2022/07/input.txt")

	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)

	type Dir struct {
		parent *Dir
		size   int64
		name   string
	}
	var first = Dir{}
	all := []*Dir{}
	var current = &first
	// mode := ""
	for scanner.Scan() {
		line := scanner.Text()

		if line[:4] == "$ cd" {
			name := line[5:]
			if name == ".." {
				current = current.parent
				continue
			}
			new := &Dir{parent: current, name: name}
			all = append(all, new)
			current = new
			continue
		}
		if line == "$ ls" {
			continue
		}
		data := strings.Split(line, " ")[0]
		if data == "dir" {
			continue
		}
		size, _ := strconv.ParseInt(data, 10, 64)

		t := current
		for t != nil {
			t.size += size
			t = t.parent
		}

		// fmt.Println(line, data, size, current)
		// fmt.Println(all)
		// fmt.Println("---------")
	}

	// part 1
	var sum int64
	for _, v := range all {
		if v.size <= 100_000 && v.name != "/" {
			sum += v.size
		}
		// fmt.Println(v)
	}
	fmt.Println(sum)
	// part 2
	max := 70_000_000
	updateSpace := 30_000_000
	free := max - int(first.size)
	needed := updateSpace - free
	fmt.Println(needed)

	bestone := -1
	for _, v := range all {
		if int(v.size) >= needed {
			if bestone == -1 || int(v.size) < bestone {
				bestone = int(v.size)
			}
		}
	}
	fmt.Println(bestone)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
