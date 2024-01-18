use std::fs;

fn main() {
    let filename = "input";
    let lines = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let str = &lines[0];

    println!("part1: {:?}", str.split(",").map(hash).sum::<usize>());

    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];
    str.split(",").for_each(|str| {
        let label: &str;
        let op: (char, usize);
        if let Some(equal) = str.split_once("=") {
            label = equal.0;
            op = ('=', equal.1.parse::<usize>().unwrap());
        } else if let Some(minus) = str.split_once("-") {
            label = minus.0;
            op = ('-', 0);
        } else {
            panic!("parse failed");
        }
        let hash = hash(label);
        let current_box = &mut boxes[hash as usize];
        let pos = current_box.iter().position(|&(l, _)| label == l);
        if op.0 == '=' {
            if pos == None {
                current_box.push((label, op.1))
            } else {
                current_box[pos.unwrap()].1 = op.1;
            }
        } else if op.0 == '-' && pos != None {
            current_box.remove(pos.unwrap());
        }
    });

    println!(
        "part2: {:?}",
        boxes
            .iter()
            .enumerate()
            .map(|(box_no, current_box)| {
                current_box
                    .iter()
                    .enumerate()
                    .map(move |(slot_no, (_, focal_length))| {
                        (box_no + 1) * (slot_no + 1) * focal_length
                    })
            })
            .flatten()
            .sum::<usize>()
    )
}

fn hash(str: &str) -> usize {
    str.as_bytes()
        .iter()
        .fold(0usize, |acc, &n| ((acc + n as usize) * 17) % 256)
}
