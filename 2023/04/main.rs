use std::fs;

fn main() {
    let filename = "demo";
    let lines: Vec<String> = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    fn parse_card_str(string: &str) -> std::collections::HashSet<u32> {
        return string
            .split_whitespace()
            .map(|item| item.parse::<u32>().unwrap())
            .collect();
    }

    let wins = lines.iter().map(|line| {
        let values = line.split_once(": ").unwrap().1;
        let (winning_str, my_str) = &values.split_once(" | ").unwrap();
        let winning_cards = parse_card_str(winning_str);
        let my_cards = parse_card_str(my_str);
        return winning_cards.intersection(&my_cards).count();
    });

    let sum: u32 = wins
        .clone()
        .map(|win| match win {
            0 => 0,
            1 => 1,
            _ => 2 << (win - 2),
        })
        .sum();

    println!("part1: {sum}");

    let mut card_counts: Vec<u32> = vec![1; wins.len()];
    for (i, win) in wins.enumerate() {
        let card_count = card_counts[i];
        for n in 0..win {
            card_counts[n + 1 + i] += card_count
        }
    }
    let sum2: u32 = card_counts.iter().sum();
    println!("part2: {sum2}");
}
