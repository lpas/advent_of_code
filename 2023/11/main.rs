use std::fs;

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

const EXPAND: usize = 1000000 - 1; // we keep the original empty space; thus we expand -1

fn main() {
    let filename = "input";
    let lines = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let space = lines.iter().map(|line| line.chars().collect()).collect();
    let (expanded_rows, expanded_cols) = expanded_space(&space);

    let galaxies = get_galaxies(space)
        .iter_mut()
        .map(|galaxie| {
            galaxie.x += expanded_cols[galaxie.x];
            galaxie.y += expanded_rows[galaxie.y];
            *galaxie
        })
        .collect();

    let galaxie_pairs = build_pairs(galaxies);
    let sum: usize = galaxie_pairs
        .iter()
        .map(|(pos1, pos2)| path_length(*pos1, *pos2))
        .sum();
    println!("part1: {sum}")
}

fn path_length(pos1: Pos, pos2: Pos) -> usize {
    pos1.x.abs_diff(pos2.x) + pos1.y.abs_diff(pos2.y)
}

fn get_galaxies(space: Vec<Vec<char>>) -> Vec<Pos> {
    space
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, char)| {
                    if *char == '#' {
                        Some(Pos { x, y })
                    } else {
                        None
                    }
                })
                .collect::<Vec<Option<Pos>>>()
        })
        .filter_map(|x| x)
        .collect()
}

fn build_pairs(vec: Vec<Pos>) -> Vec<(Pos, Pos)> {
    let mut pairs = vec![];
    let length: usize = vec.len();
    for (n, &item) in vec.iter().enumerate() {
        for x in (n + 1)..length {
            pairs.push((item, vec[x]));
        }
    }
    pairs
}

fn empty_space(space: &Vec<Vec<char>>) -> (Vec<bool>, Vec<bool>) {
    let width = space[0].len();
    let height = space.len();

    let mut empty_rows = vec![true; height];
    let mut empty_cols = vec![true; width];

    for y in 0..height {
        for x in 0..width {
            if space[y][x] == '#' {
                empty_rows[y] = false;
                empty_cols[x] = false;
            }
        }
    }
    (empty_rows, empty_cols)
}

fn expanded_space(space: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let (empty_rows, empty_cols) = empty_space(space);
    return (expand(empty_rows), expand(empty_cols));
}

fn expand(vec: Vec<bool>) -> Vec<usize> {
    let mut expand = 0;
    vec.iter()
        .map(|&empty| {
            if empty {
                expand += EXPAND;
            }
            return expand;
        })
        .collect()
}
