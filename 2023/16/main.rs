use std::cmp::max;
use std::iter::FromIterator;
use std::{collections::HashSet, fs};

#[derive(Debug, Hash, Clone, PartialEq, Eq, Copy)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Pos {
    x: usize,
    y: usize,
    direction: Direction,
}

fn main() {
    let filename = "input";
    let lines = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let part1 = energized_tiles(
        &grid,
        Pos {
            x: 0,
            y: 0,
            direction: Direction::RIGHT,
        },
    );
    println!("part1: {part1}");

    let mut part2 = 0usize;
    let width = grid[0].len();
    let height = grid.len();

    for y in 0..height {
        part2 = max(
            part2,
            energized_tiles(
                &grid,
                Pos {
                    x: 0,
                    y,
                    direction: Direction::RIGHT,
                },
            ),
        );
        part2 = max(
            part2,
            energized_tiles(
                &grid,
                Pos {
                    x: width - 1,
                    y,
                    direction: Direction::LEFT,
                },
            ),
        );
    }
    for x in 0..width {
        part2 = part2.max(energized_tiles(
            &grid,
            Pos {
                x,
                y: 0,
                direction: Direction::DOWN,
            },
        ));
        part2 = part2.max(energized_tiles(
            &grid,
            Pos {
                x,
                y: height - 1,
                direction: Direction::UP,
            },
        ));
    }
    println!("part2: {part2}");
}

fn energized_tiles(grid: &Vec<Vec<char>>, start_pos: Pos) -> usize {
    let mut visisted: HashSet<Pos> = HashSet::new();
    let mut queue: Vec<Pos> = vec![];
    queue.push(start_pos);

    let width = grid[0].len();
    let height = grid.len();

    while let Some(current) = queue.pop() {
        visisted.insert(current);
        let char = grid[current.y][current.x];
        // println!("{current:?}, {char}");

        let new_directions = match char {
            '.' => vec![current.direction],
            '|' if current.direction == Direction::UP || current.direction == Direction::DOWN => {
                vec![current.direction]
            }
            '|' => vec![Direction::UP, Direction::DOWN],
            '-' if current.direction == Direction::LEFT
                || current.direction == Direction::RIGHT =>
            {
                vec![current.direction]
            }
            '-' => vec![Direction::LEFT, Direction::RIGHT],
            '/' if current.direction == Direction::UP => vec![Direction::RIGHT],
            '/' if current.direction == Direction::RIGHT => vec![Direction::UP],
            '/' if current.direction == Direction::DOWN => vec![Direction::LEFT],
            '/' if current.direction == Direction::LEFT => vec![Direction::DOWN],
            '\\' if current.direction == Direction::UP => vec![Direction::LEFT],
            '\\' if current.direction == Direction::RIGHT => vec![Direction::DOWN],
            '\\' if current.direction == Direction::DOWN => vec![Direction::RIGHT],
            '\\' if current.direction == Direction::LEFT => vec![Direction::UP],
            _ => vec![],
        };

        for direction in new_directions {
            if let Some(new_pos) = match direction {
                Direction::UP if current.y > 0 => Some(Pos {
                    x: current.x,
                    y: current.y - 1,
                    direction,
                }),
                Direction::RIGHT if current.x < width - 1 => Some(Pos {
                    x: current.x + 1,
                    y: current.y,
                    direction,
                }),
                Direction::DOWN if current.y < height - 1 => Some(Pos {
                    x: current.x,
                    y: current.y + 1,
                    direction,
                }),
                Direction::LEFT if current.x > 0 => Some(Pos {
                    x: current.x - 1,
                    y: current.y,
                    direction,
                }),
                _ => None,
            } {
                if !visisted.contains(&new_pos) {
                    queue.push(new_pos);
                }
            }
        }
    }

    HashSet::<Pos>::from_iter(visisted.iter().map(|pos| Pos {
        x: pos.x,
        y: pos.y,
        direction: Direction::DOWN,
    }))
    .len()
}

#[allow(dead_code)]
fn print(vec: &Vec<Vec<char>>) {
    for line in vec {
        println!("{}", line.iter().collect::<String>());
    }
}
