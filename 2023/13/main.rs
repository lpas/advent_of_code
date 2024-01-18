use std::fs;

fn main() {
    let filename = "input";
    let lines = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let mut grid = vec![];

    let mut part_1_values = vec![];
    for line in lines.iter() {
        if line == "" {
            part_1_values.push(get_value_1(&grid));
            grid = vec![];
        } else {
            grid.push(line.chars().collect::<Vec<char>>())
        }
    }
    part_1_values.push(get_value_1(&grid));
    println!("part1 {}", part_1_values.iter().sum::<i32>());

    // part2

    let mut part_2_values = vec![];
    grid = vec![];
    for line in lines.iter() {
        if line == "" {
            part_2_values.push(get_value_2(grid));
            grid = vec![];
        } else {
            grid.push(line.chars().collect::<Vec<char>>())
        }
    }
    part_2_values.push(get_value_2(grid));
    println!("part2 {}", part_2_values.iter().sum::<i32>());
}

fn get_value_2(mut vec: Vec<Vec<char>>) -> i32 {
    let mut fliped: Vec<Vec<char>> = flip(&vec);

    let row_v = get_mirror_row(&vec, -1);
    let col_v = get_mirror_row(&fliped, -1);

    for y in 0..vec.len() {
        for x in 0..vec[0].len() {
            let tmp = vec[y][x];
            let new = if tmp == '#' { '.' } else { '#' };
            vec[y][x] = new;
            fliped[x][y] = new;

            let new_row = get_mirror_row(&vec, row_v);
            let new_col = get_mirror_row(&fliped, col_v);

            if new_row != 0 {
                return new_row * 100;
            }
            if new_col != 0 {
                return new_col;
            }

            vec[y][x] = tmp;
            fliped[x][y] = tmp;
        }
    }
    return 0;
}

fn get_value_1(vec: &Vec<Vec<char>>) -> i32 {
    let fliped: Vec<Vec<char>> = flip(&vec);
    let row_v = get_mirror_row(&vec, -1);
    let col_v = get_mirror_row(&fliped, -1);
    return row_v * 100 + col_v;
}

fn flip(vec: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = vec![];
    for n in 0..vec[0].len() {
        let mut new_line = vec![];
        for line in vec {
            new_line.push(line[n])
        }
        result.push(new_line);
    }

    result
}

#[allow(dead_code)]
fn print(vec: &Vec<Vec<char>>) {
    for line in vec {
        println!("{}", line.iter().collect::<String>())
    }
}

fn get_mirror_row(vec: &Vec<Vec<char>>, not_allowed: i32) -> i32 {
    for (n, line) in vec.iter().enumerate() {
        if n + 1 == vec.len() {
            break;
        }
        for (n2, line2) in vec.iter().skip(n + 1).enumerate() {
            let possible: i32 = (n + n2 / 2) as i32;
            if possible + 1 != not_allowed && line == line2 && check(vec, possible as usize) {
                return possible + 1;
            }
        }
    }
    return 0;
}

fn check(vec: &Vec<Vec<char>>, n: usize) -> bool {
    let length = vec.len();
    let mut i = n;
    let mut j = n + 1;
    loop {
        if vec[i] != vec[j] {
            return false;
        }
        if i == 0 || j + 1 == length {
            break;
        }
        i -= 1;
        j += 1;
    }
    return true;
}
