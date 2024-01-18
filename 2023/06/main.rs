use std::fs;

fn main() {
    let filename = "input";
    let lines = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    fn parse_line(line: &str) -> Vec<u32> {
        line.split_once(":")
            .unwrap()
            .1
            .split_whitespace()
            .map(|item| item.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
    }

    let times = parse_line(&lines[0]);
    let distances = parse_line(&lines[1]);

    let result = times
        .iter()
        .zip(distances.clone().into_iter())
        .map(|(time, distance)| {
            return (1..*time)
                .map(|n| (time - n) * n)
                .filter(|n| n > &distance)
                .count();
        })
        .reduce(|a, b| a * b);
    println!("{result:?}");

    fn to_single_num(vec: Vec<u32>) -> u64 {
        vec.iter()
            .map(|n| n.to_string())
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
    }

    let time2 = to_single_num(times);
    let distance2 = to_single_num(distances);

    let min = (1..time2).find(|n| (time2 - n) * n > distance2).unwrap();
    let max = (1..time2)
        .rev()
        .find(|n| (time2 - n) * n > distance2)
        .unwrap();
    let result2 = max - min + 1;

    println!("{result2:?}")
}
