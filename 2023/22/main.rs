use std::collections::VecDeque;
use std::iter::FromIterator;
use std::{collections::HashSet, fs, ops::RangeInclusive};

#[derive(Debug, Eq, PartialEq, Hash)]
struct Brick {
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
    z: RangeInclusive<usize>,
}

fn main() {
    let filename = "input";
    let lines = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let mut bricks = parse_input(&lines);
    bricks.sort_by(|a, b| a.z.start().cmp(b.z.start()));

    let mut max_x = 0usize;
    let mut max_y = 0usize;
    let mut max_z = 0usize;

    for brick in bricks.iter() {
        max_x = max_x.max(*brick.x.end());
        max_y = max_y.max(*brick.y.end());
        max_z = max_z.max(*brick.z.end());
    }

    let mut grid = vec![vec![vec![0usize; max_z + 1]; max_y + 1]; max_x + 1];

    let settled_bricks: Vec<Brick> = bricks
        .iter()
        .enumerate()
        .map(|(id, brick)| {
            let mut z_start = *brick.z.start();
            'z_loop: for z in (1..*brick.z.start()).rev() {
                for x in brick.x.clone() {
                    for y in brick.y.clone() {
                        if grid[x][y][z] > 0 {
                            break 'z_loop;
                        }
                    }
                }
                z_start = z;
            }

            let z_end = brick.z.end() - (brick.z.start() - z_start);
            for z in z_start..=z_end {
                for x in brick.x.clone() {
                    for y in brick.y.clone() {
                        grid[x][y][z] = id + 1;
                    }
                }
            }

            Brick {
                x: brick.x.clone(),
                y: brick.y.clone(),
                z: z_start..=z_end,
            }
        })
        .collect();

    let on_top_of: Vec<Vec<usize>> = settled_bricks
        .iter()
        .map(|brick| {
            let z = brick.z.start() - 1;
            if z == 0 {
                return vec![];
            }
            let mut set: HashSet<usize> = HashSet::new();
            for x in brick.x.clone() {
                for y in brick.y.clone() {
                    set.insert(grid[x][y][z]);
                }
            }
            set.remove(&0);
            return Vec::from_iter(set.iter().map(|id| id - 1));
        })
        .collect();

    let part1 = settled_bricks.len()
        - on_top_of
            .iter()
            .filter_map(|vec| if vec.len() == 1 { Some(vec[0]) } else { None })
            .collect::<HashSet<usize>>()
            .len();

    println!("part1: {part1}");

    let mut supports: Vec<Vec<usize>> = vec![vec![]; settled_bricks.len()];

    on_top_of.iter().enumerate().for_each(|(idx, vec)| {
        for &c in vec {
            supports[c].push(idx);
        }
    });

    let part2: usize = (0..settled_bricks.len())
        .map(|idx| bricks_will_fall(idx, &supports, &on_top_of))
        .sum();
    println!("part2: {part2:?}");
}

fn parse_input(lines: &Vec<String>) -> Vec<Brick> {
    lines
        .iter()
        .map(|line| {
            let (from, to) = line.split_once("~").unwrap();
            let from_cords: Vec<usize> = from
                .split(",")
                .map(|num| num.parse::<usize>().unwrap())
                .collect();
            let to_cords: Vec<usize> = to
                .split(",")
                .map(|num| num.parse::<usize>().unwrap())
                .collect();

            Brick {
                x: from_cords[0]..=to_cords[0],
                y: from_cords[1]..=to_cords[1],
                z: from_cords[2]..=to_cords[2],
            }
        })
        .collect()
}

fn bricks_will_fall(
    brick_idx: usize,
    supports: &Vec<Vec<usize>>,
    on_top_of: &Vec<Vec<usize>>,
) -> usize {
    if supports[brick_idx].is_empty() {
        return 0;
    }
    let mut fallen_indexes = HashSet::<usize>::new();
    let mut queue = VecDeque::<usize>::new();
    queue.push_back(brick_idx);

    while let Some(idx) = queue.pop_front() {
        fallen_indexes.insert(idx);

        for &support_idx in supports[idx].iter() {
            if on_top_of[support_idx]
                .iter()
                .all(|idx| fallen_indexes.contains(idx))
            {
                queue.push_back(support_idx)
            }
        }
    }

    return fallen_indexes.len() - 1;
}
