package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"reflect"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

type Node struct {
	elements []*Node
	value    int
	parent   *Node
}

func (f Node) String() string {
	elems := make([]string, 0)
	if f.value != -1 {
		return fmt.Sprint(f.value)
	}
	for _, item := range f.elements {
		if item.value != -1 {
			elems = append(elems, fmt.Sprint(item.value))
		}
		if item.elements != nil {
			elems = append(elems, item.String())
		}
	}
	return "[" + strings.Join(elems, ",") + "]"
}

func main() {
	f, err := os.Open("./2022/13/input.txt")

	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)

	getInt := func(val string) int {
		value, _ := strconv.ParseInt(val, 10, 64)
		return int(value)
	}

	re := regexp.MustCompile(`(\[*)(\d*)(\]*)`)
	parse := func(line string) Node {
		items := strings.Split(line, ",")
		current := &Node{value: -1, elements: nil}
		for _, item := range items {
			match := re.FindStringSubmatch(item)

			for range match[1] { // [
				node := &Node{value: -1, elements: nil, parent: current}
				current.elements = append(current.elements, node)
				current = node
			}
			if match[2] != "" {
				current.elements = append(current.elements,
					&Node{value: getInt(match[2]), elements: nil})
			}

			for range match[3] { // ]
				current = current.parent // todo this doesn't work as i would expected!
			}
		}
		return *current
	}

	BoolAddr := func(b bool) *bool {
		boolVar := b
		return &boolVar
	}

	var compare func(l1 Node, l2 Node, d int) *bool
	compare = func(l1 Node, l2 Node, d int) *bool {
		// fmt.Printf("(%v) Compare %v vs %v \n", d, l1, l2)
		if l1.value != -1 && l2.value == -1 {
			node := &Node{value: -1, parent: l1.parent}
			l1.parent = node
			node.elements = append(node.elements, &l1)
			// fmt.Printf("Mixed types; convert left to %v and retry comparison \n", node)
			return compare(*node, l2, d)
		}
		if l1.value == -1 && l2.value != -1 {
			node := &Node{value: -1, parent: l2.parent}
			l1.parent = node
			node.elements = append(node.elements, &l2)
			// fmt.Printf("Mixed types; convert right to %v and retry comparison \n", node)
			return compare(l1, *node, d)
		}
		if l1.value != -1 && l2.value != -1 {
			if l1.value == l2.value {
				return nil
			}
			return BoolAddr(l1.value < l2.value)
		}
		for index := range l1.elements {
			if len(l2.elements)-1 < index {
				return BoolAddr(false)
			}
			r := compare(*l1.elements[index], *l2.elements[index], d+1)
			if r != nil {
				return r
			}
		}
		if len(l2.elements) == len(l1.elements) {
			return nil
		}
		return BoolAddr(len(l2.elements) > len(l1.elements))

	}

	result := 0
	index := 1
	all := make([]Node, 0)
	for scanner.Scan() {
		line1 := scanner.Text()
		scanner.Scan()
		line2 := scanner.Text()
		scanner.Scan() // blank
		// fmt.Println("parsed:", line1, line2)
		parse1 := *parse(line1).elements[0]
		parse2 := *parse(line2).elements[0]

		if *compare(parse1, parse2, 0) == true {
			result += index
		}
		index++
		all = append(all, parse1, parse2)

	}

	fmt.Println(result) // part 1

	packet1 := *parse("[[2]]").elements[0]
	packet2 := *parse("[[6]]").elements[0]
	all = append(all, packet1, packet2)

	sort.SliceStable(all, func(i, j int) bool {
		return *compare(all[i], all[j], 0) == true
	})

	index1 := 0
	index2 := 0
	for index, item := range all {
		if reflect.DeepEqual(packet1, item) {
			index1 = index + 1
		}
		if reflect.DeepEqual(packet2, item) {
			index2 = index + 1
		}
		// fmt.Println(item)
	}

	fmt.Println(index1, index2, index1*index2)
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
