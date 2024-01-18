use std::iter::FromIterator;
use std::{collections::HashSet, fs};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos { x, y }
    }
}

fn main() {
    let filename = "input";
    let lines = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let height = lines.len() as i64;
    let width = lines[0].len() as i64;
    let in_grid = |x: i64, y: i64| (x >= 0 && x < width && y >= 0 && y < height);
    let to_grid_pos = |vec: Vec<(i64, i64)>| {
        vec.into_iter()
            .filter(|(x, y)| in_grid(*x, *y))
            .map(|(x, y)| Pos::new(x as usize, y as usize))
            .collect::<Vec<Pos>>()
    };
    let char_to_grid_pos = |c: char, x: i64, y: i64| match c {
        '|' => to_grid_pos(vec![(x, y + 1), (x, y - 1)]),
        '-' => to_grid_pos(vec![(x + 1, y), (x - 1, y)]),
        'L' => to_grid_pos(vec![(x, y - 1), (x + 1, y)]),
        'J' => to_grid_pos(vec![(x, y - 1), (x - 1, y)]),
        '7' => to_grid_pos(vec![(x, y + 1), (x - 1, y)]),
        'F' => to_grid_pos(vec![(x, y + 1), (x + 1, y)]),
        '.' => vec![],
        _ => vec![],
    };

    let mut start = Pos::new(0 as usize, 0 as usize);
    let mut cs = lines
        .iter()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut grid: Vec<Vec<Vec<Pos>>> = lines
        .into_iter()
        .enumerate()
        .map(|(y, line)| (y as i64, line))
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| (x as i64, c))
                .map(|(x, c)| match c {
                    'S' => {
                        start = Pos::new(x as usize, y as usize);
                        vec![]
                    }
                    _ => char_to_grid_pos(c, x, y),
                })
                .collect()
        })
        .collect();

    // replaceing start marker with positions
    let start_x = start.x as i64;
    let start_y = start.y as i64;
    grid[start.y][start.x] = to_grid_pos(vec![
        (start_x + 1, start_y),
        (start_x - 1, start_y),
        (start_x, start_y + 1),
        (start_x, start_y - 1),
    ])
    .into_iter()
    .filter(|pos| grid[pos.y][pos.x].iter().filter(|p| p == &&start).count() > 0)
    .collect();
    // replacing S with real pipe symbol
    let start_hashset: HashSet<Pos> = HashSet::from_iter(grid[start.y][start.x].clone());
    cs[start.y][start.x] = "|-LJ7F"
        .chars()
        .find(|c| HashSet::from_iter(char_to_grid_pos(*c, start_x, start_y)) == start_hashset)
        .unwrap();

    let mut from = &start;
    let mut curr = &grid[start.y][start.x][0];
    let mut count = 1;
    let mut pipes: Vec<Vec<bool>> = (0..height).map(|_| vec![false; width as usize]).collect();
    pipes[start.y][start.x] = true;
    loop {
        pipes[curr.y][curr.x] = true;
        count += 1;
        let curr_directions = &grid[curr.y][curr.x];
        let next = if &curr_directions[0] == from {
            &curr_directions[1]
        } else {
            &curr_directions[0]
        };
        if next == &start {
            break;
        }

        from = curr;
        curr = next;
    }
    let result = count / 2;
    println!("part1: {result}");

    // we check per winding order if we are inside the pipe poly
    // crossing a vertical line flips us from outside to inside
    // and the next flips us back to outside
    // we count all pipes that face north as vertical
    let mut sum = 0;
    for y in 0..(height as usize) {
        let mut inside = false;
        for x in 0..(width as usize) {
            let char = cs[y][x];
            let vertical_pipe = char == '|' || char == 'J' || char == 'L';
            if pipes[y][x] {
                if vertical_pipe {
                    inside = !inside;
                }
            } else if inside {
                sum += 1;
            }
        }
    }
    println!("part2: {sum}")
}
