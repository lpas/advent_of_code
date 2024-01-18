use std::{collections::HashSet, fs, ops::RangeInclusive};

const PART1: bool = false;
fn main() {
    let filename = "input";
    let lines = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let data: Vec<_> = lines
        .iter()
        .map(|line| {
            let split: Vec<&str> = line.split_ascii_whitespace().collect();
            if PART1 {
                let direction = split[0];
                let meter = split[1].parse::<i32>().unwrap();
                return (direction.chars().nth(0).unwrap(), meter);
            } else {
                let hex = split[2].chars();
                let m = i32::from_str_radix(&hex.clone().skip(2).take(5).collect::<String>(), 16)
                    .unwrap();
                let d = match hex.clone().nth(7).unwrap() {
                    '0' => 'R',
                    '1' => 'D',
                    '2' => 'L',
                    '3' => 'U',
                    _ => panic!(),
                };
                return (d, m);
            }
        })
        .collect();

    let (x_ranges, y_ranges) = get_x_y_ranges(&data);

    let width = x_ranges.len();
    let height = y_ranges.len();
    let mut grid = vec![vec![false; width]; height];

    grid_outline(&mut grid, &y_ranges, &x_ranges, data);

    // get some point in the middle to start the fill
    let y = height / 2;
    let line = &grid[y];
    let x = first_fill_in_line(line);

    let filled_grid = get_filled_grid(grid, y, x);

    let mut sum = 0;
    let x_counts: Vec<usize> = x_ranges.into_iter().map(|item| item.count()).collect();
    let y_counts: Vec<usize> = y_ranges.into_iter().map(|item| item.count()).collect();
    for (y, line) in filled_grid.iter().enumerate() {
        for (x, filled) in line.iter().enumerate() {
            if *filled {
                sum += x_counts[x] * y_counts[y];
            }
        }
    }
    println!("{sum}");
}

fn get_filled_grid(grid: Vec<Vec<bool>>, y: usize, x: usize) -> Vec<Vec<bool>> {
    let mut filled_grid = grid.clone();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: Vec<(usize, usize)> = vec![];
    queue.push((y, x));
    while let Some(item) = queue.pop() {
        if visited.contains(&item) {
            continue;
        }
        visited.insert(item);
        filled_grid[item.0][item.1] = true;
        for possible_new_one in vec![
            (item.0 + 1, item.1),
            (item.0 - 1, item.1),
            (item.0, item.1 + 1),
            (item.0, item.1 - 1),
        ] {
            if !grid[possible_new_one.0][possible_new_one.1] {
                queue.push(possible_new_one)
            }
        }
    }
    filled_grid
}

fn grid_outline(
    grid: &mut Vec<Vec<bool>>,
    y_ranges: &Vec<RangeInclusive<i32>>,
    x_ranges: &Vec<RangeInclusive<i32>>,
    data: Vec<(char, i32)>,
) {
    let mut last_x = 0;
    let mut last_y = 0;
    grid[get_indices_for_ranges(0, 0, y_ranges)[0]][get_indices_for_ranges(0, 0, x_ranges)[0]] =
        true;
    let mut x = 0;
    let mut y = 0;
    for &(direction, meter) in data.iter() {
        match direction {
            'U' => y -= meter,
            'R' => x += meter,
            'D' => y += meter,
            'L' => x -= meter,
            _ => (),
        }

        let xr = get_indices_for_ranges(last_x, x, x_ranges);
        let yr = get_indices_for_ranges(last_y, y, y_ranges);
        for &grid_x in xr.iter() {
            for &grid_y in yr.iter() {
                grid[grid_y][grid_x] = true;
            }
        }

        last_x = x;
        last_y = y;
    }
}

fn get_x_y_ranges(
    data: &Vec<(char, i32)>,
) -> (
    Vec<std::ops::RangeInclusive<i32>>,
    Vec<std::ops::RangeInclusive<i32>>,
) {
    let mut x = 0;
    let mut y = 0;

    let mut x_positions = HashSet::new();
    let mut y_positions = HashSet::new();
    x_positions.insert(x);
    y_positions.insert(y);

    for &(direction, meter) in data.iter() {
        match direction {
            'U' => y -= meter,
            'R' => x += meter,
            'D' => y += meter,
            'L' => x -= meter,
            _ => (),
        }
        x_positions.insert(x);
        y_positions.insert(y);
    }

    let mut x_positions = x_positions.into_iter().collect::<Vec<i32>>();
    x_positions.sort();
    let mut y_positions = y_positions.into_iter().collect::<Vec<i32>>();
    y_positions.sort();
    (get_range(&x_positions), get_range(&y_positions))
}

fn get_range(positions: &Vec<i32>) -> Vec<std::ops::RangeInclusive<i32>> {
    let mut x_ranges = vec![];
    let mut last = 0;
    for &x in positions {
        if last != x - 1 {
            x_ranges.push((last + 1)..=(x - 1));
        }
        x_ranges.push(x..=x);
        last = x;
    }
    x_ranges.remove(0);
    return x_ranges;
}

fn first_fill_in_line(line: &Vec<bool>) -> usize {
    let mut seen_hash = false;
    for (x, value) in line.iter().enumerate() {
        if seen_hash && !value {
            return x;
        } else if *value {
            seen_hash = true
        }
    }
    return 0usize;
}

fn get_indices_for_ranges(last: i32, new: i32, ranges: &Vec<RangeInclusive<i32>>) -> Vec<usize> {
    create_ranges(last, new)
        .iter()
        .flat_map(|item| indices(ranges, item))
        .collect::<Vec<usize>>()
}

fn create_ranges(last: i32, new: i32) -> Vec<RangeInclusive<i32>> {
    if last == new || new - 1 == last || new == last - 1 {
        return vec![new..=new];
    }
    if new < last {
        return vec![(new + 1)..=(last - 1), new..=new];
    } else {
        return vec![(last + 1)..=(new - 1), new..=new];
    }
}

fn indices(vec: &Vec<RangeInclusive<i32>>, range: &RangeInclusive<i32>) -> Vec<usize> {
    vec.iter()
        .enumerate()
        .filter_map(|(index, value)| {
            if value.start() >= range.start() && value.end() <= range.end() {
                Some(index)
            } else {
                None
            }
        })
        .collect()
}

#[allow(dead_code)]
fn write_file(path: &str, grid: &Vec<Vec<bool>>) {
    let data = grid
        .iter()
        .map(|line| {
            line.iter()
                .map(|&b| if b { '#' } else { '.' })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n");

    fs::write(path, data).expect("Unable to write file");
}

#[allow(dead_code)]
fn print(grid: &Vec<Vec<bool>>) {
    for line in grid {
        println!(
            "{}",
            line.iter()
                .map(|&b| if b { '#' } else { '.' })
                .collect::<String>()
        )
    }
}
