package main

import (
	"bufio"
	"errors"
	"fmt"
	"log"
	"math"
	"os"
	"regexp"
	"sort"
	"strconv"
)

func getInt(val string) int {
	value, _ := strconv.ParseInt(val, 10, 64)
	return int(value)
}

type Range struct {
	start int
	end   int
}

func (r *Range) length() int {
	if r.start < 0 && r.end > 0 {
		return r.start*-1 + r.end + 1
	} else {
		return r.end - r.start + 1
	}
}

func main() {
	f, err := os.Open("./2022/15/input.txt")

	if err != nil {
		log.Fatal(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)

	type Pos struct {
		x int
		y int
	}

	type Sensor struct {
		pos            Pos
		closestBeacon  Pos
		beaconDistance int
	}
	r, _ := regexp.Compile("Sensor at x=(-?[0-9]*), y=(-?[0-9]*): closest beacon is at x=(-?[0-9]*), y=(-?[0-9]*)")

	var sensors []Sensor
	for scanner.Scan() {
		line := scanner.Text()
		match := r.FindStringSubmatch(line)
		sensors = append(sensors,
			Sensor{
				pos:           Pos{x: getInt(match[1]), y: getInt(match[2])},
				closestBeacon: Pos{x: getInt(match[3]), y: getInt(match[4])},
			})

	}

	distance := func(pos1, pos2 Pos) int {
		return int(math.Abs(float64(pos1.x-pos2.x)) + math.Abs(float64(pos1.y-pos2.y)))
	}

	beaconDistance := func(sensor Sensor) int {
		return distance(sensor.pos, sensor.closestBeacon)
	}

	min := sensors[0].pos
	max := Pos{0, 0}
	for i, sensor := range sensors {
		sensors[i].beaconDistance = beaconDistance(sensor)
		d := sensors[i].beaconDistance
		if minX := sensor.pos.x - d; minX < min.x {
			min.x = minX
		}
		if minY := sensor.pos.y - d; minY < min.y {
			min.y = minY
		}

		if maxX := sensor.pos.x + d; maxX > max.x {
			max.x = maxX
		}
		if maxY := sensor.pos.y + d; maxY > max.y {
			max.y = maxY
		}
	}

	var NoRange = errors.New("no range")
	getRange := func(sensor Sensor, y int) (Range, error) {
		offset := sensor.beaconDistance - int(math.Abs(float64(sensor.pos.y-y)))
		if offset < 0 {
			return Range{}, NoRange
		}
		return Range{sensor.pos.x - offset, sensor.pos.x + offset}, nil
	}

	rangesForY := func(y int) (merged_ranges []Range) {
		var ranges []Range
		for _, sensor := range sensors {
			r, e := getRange(sensor, y)
			if e != nil {
				continue
			}
			ranges = append(ranges, r)
		}
		sort.Slice(ranges, func(i, j int) bool {
			return ranges[i].start < ranges[j].start
		})
		merged_ranges = append(merged_ranges, ranges[0])
		for _, r := range ranges[1:] {
			last_merged := merged_ranges[len(merged_ranges)-1]
			if r.start <= last_merged.end || last_merged.end+1 == r.start {
				if r.end > last_merged.end {
					new := Range{last_merged.start, r.end}
					merged_ranges[len(merged_ranges)-1] = new
				}
			} else {
				merged_ranges = append(merged_ranges, r)
			}
		}
		return

	}

	// part1
	y := 10
	y = 2000000
	ranges := rangesForY(y)
	count := 0
	for _, r := range ranges {
		count += r.length()
	}
	// set of x posistions where beacons are already set
	set := make(map[int]bool)
	for _, sensor := range sensors {
		if sensor.closestBeacon.y == y {
			set[sensor.closestBeacon.x] = true
		}
	}
	fmt.Println(count - len(set))

	// pars2
	yMax := 20
	yMax = 4000000
	for y := 0; y <= yMax; y++ {
		ranges := rangesForY(y)
		if len(ranges) > 1 {
			pos := Pos{ranges[0].end + 1, y}
			fmt.Println(pos, pos.x*4000000+pos.y)
			break
		}
	}

	// print grid
	// checkPos := func(pos Pos) bool {
	// 	for _, sensor := range sensors {
	// 		if sensor.closestBeacon.x == pos.x && sensor.closestBeacon.y == pos.y {
	// 			return false
	// 		}
	// 		if distance(pos, sensor.pos) <= sensor.beaconDistance {
	// 			return false
	// 		}
	// 	}
	// 	return true
	// }

	// for y := min.y; y <= max.y; y++ {
	// 	for x := min.x; x <= max.x; x++ {
	// 		if !checkPos(Pos{x, y}) {
	// 			fmt.Print("#")
	// 		} else {
	// 			fmt.Print(".")
	// 		}
	// 	}
	// 	fmt.Print("\n")
	// }

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
