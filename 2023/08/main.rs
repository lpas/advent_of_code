use std::{collections::HashMap, fs};

fn main() {
    let filename = "input";
    let lines = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let moves: Vec<char> = lines[0].chars().collect();

    let network: HashMap<String, (String, String)> = lines
        .iter()
        .skip(2)
        .map(|str| {
            let chars: Vec<char> = str.chars().collect();
            return (
                chars[0..3].iter().collect::<String>(),
                (
                    chars[7..10].iter().collect::<String>(),
                    chars[12..15].iter().collect::<String>(),
                ),
            );
        })
        .collect();

    let get_next = |key: &String, current_move: char| {
        let tuple = network.get(key).unwrap();
        if current_move == 'L' {
            &tuple.0
        } else {
            &tuple.1
        }
    };

    #[allow(unused_variables)]
    let part1 = || {
        let mut index = &"AAA".to_string();
        let mut move_index = 0;
        let moves_length = moves.len();
        loop {
            let current_move = moves[move_index % moves_length];
            move_index += 1;
            index = get_next(index, current_move);
            if index == "ZZZ" {
                break;
            }
        }
        println!("{move_index}")
    };

    let part2 = || {
        let curr: Vec<&String> = network
            .keys()
            .filter(|key| key.chars().nth(2).unwrap() == 'A')
            .collect();

        let moves_length = moves.len();

        let result = curr
            .into_iter()
            .map(|mut key| {
                let mut move_index = 0;
                let mut start = 0;
                loop {
                    let current_move = moves[move_index % moves_length];
                    move_index += 1;
                    key = get_next(key, current_move);

                    if key.chars().last().unwrap() == 'Z' {
                        if start == 0 {
                            start = move_index
                        } else {
                            return move_index - start;
                        }
                    }
                }
            })
            .reduce(|a, b| lcm(a, b));
        println!("{result:?}");
    };

    // part1();
    part2();
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b > 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }

    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
