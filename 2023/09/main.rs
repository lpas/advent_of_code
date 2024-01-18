use std::fs;

fn main() {
    let filename = "input";
    let lines = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let line_numbers = lines
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|item| item.parse::<i64>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i64>>>();

    fn diffs(vec: &Vec<i64>) -> Vec<i64> {
        return vec
            .iter()
            .zip(vec.iter().skip(1))
            .map(|(left, right)| right - left)
            .collect();
    }

    fn get_last(vec: Vec<i64>) -> i64 {
        let last = vec.iter().last().unwrap();
        if vec.iter().all(|item| *item == 0) {
            return *last;
        }
        return get_last(diffs(&vec)) + last;
    }

    let result: i64 = line_numbers.iter().map(|line| get_last(line.clone())).sum();
    println!("part1 {:?}", result);
    let result: i64 = line_numbers
        .iter()
        .map(|line| get_last(line.clone().into_iter().rev().collect()))
        .sum();
    println!("part2 {:?}", result);
}
