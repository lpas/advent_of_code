use std::{collections::HashMap, fs};

fn main() {
    let filename = "input";
    let lines = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let mut grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let width = grid[0].len();
    let height = grid.len();

    north(height, width, &mut grid);
    println!("part1: {:?}", total_load(&grid));

    let mut grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let mut set: HashMap<String, i32> = HashMap::new();
    let cycles = 1000000000;
    for cycle in 1..(cycles + 1) {
        north(height, width, &mut grid);
        west(width, height, &mut grid);
        south(height, width, &mut grid);
        east(width, height, &mut grid);

        let key: String = grid.iter().flatten().collect();
        if set.contains_key(&key) {
            let start_cycle = set.get(&key).unwrap();
            let cycle_length = cycle - start_cycle;
            let pos = ((cycles - start_cycle) % cycle_length) + start_cycle;

            for (key, value) in set {
                if value == pos {
                    let part2_grid: Vec<Vec<char>> = key
                        .chars()
                        .collect::<Vec<char>>()
                        .chunks(width)
                        .map(|c| c.to_vec())
                        .collect();
                    println!("part2: {}", total_load(&part2_grid));
                    break;
                }
            }
            break;
        }
        set.insert(key, cycle);
    }
}

fn north(height: usize, width: usize, grid: &mut Vec<Vec<char>>) {
    for y in 1..height {
        for x in 0..width {
            if grid[y][x] == 'O' {
                let mut n = 0;
                loop {
                    if n < y && grid[y - n - 1][x] == '.' {
                        n += 1
                    } else {
                        break;
                    }
                }
                if n > 0 {
                    grid[y - n][x] = 'O';
                    grid[y][x] = '.';
                }
            }
        }
    }
}

fn west(width: usize, height: usize, grid: &mut Vec<Vec<char>>) {
    for x in 1..width {
        for y in 0..height {
            if grid[y][x] == 'O' {
                let mut n = 0;
                loop {
                    if n < x && grid[y][x - n - 1] == '.' {
                        n += 1;
                    } else {
                        break;
                    }
                }
                if n > 0 {
                    grid[y][x - n] = 'O';
                    grid[y][x] = '.';
                }
            }
        }
    }
}

fn south(height: usize, width: usize, grid: &mut Vec<Vec<char>>) {
    for y in (0..height - 1).rev() {
        for x in 0..width {
            if grid[y][x] == 'O' {
                let mut n = 0;
                loop {
                    if y + n < height - 1 && grid[y + n + 1][x] == '.' {
                        n += 1
                    } else {
                        break;
                    }
                }
                if n > 0 {
                    grid[y + n][x] = 'O';
                    grid[y][x] = '.';
                }
            }
        }
    }
}

fn east(width: usize, height: usize, grid: &mut Vec<Vec<char>>) {
    for x in (0..width - 1).rev() {
        for y in 0..height {
            if grid[y][x] == 'O' {
                let mut n = 0;
                loop {
                    if x + n < width - 1 && grid[y][x + n + 1] == '.' {
                        n += 1;
                    } else {
                        break;
                    }
                }
                if n > 0 {
                    grid[y][x + n] = 'O';
                    grid[y][x] = '.';
                }
            }
        }
    }
}

fn total_load(grid: &Vec<Vec<char>>) -> usize {
    grid.iter()
        .map(|line| line.iter().map(|&c| c == 'O').filter(|&c| c).count())
        .rev()
        .enumerate()
        .map(|(i, c)| (i + 1) * c)
        .sum::<usize>()
}

#[allow(dead_code)]
fn print(vec: &Vec<Vec<char>>) {
    for line in vec {
        println!("{}", line.iter().collect::<String>())
    }
}
