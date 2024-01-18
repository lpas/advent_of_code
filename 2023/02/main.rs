use std::cmp::max;
use std::fs;

fn part1() {
    let filename = "input";
    let lines: Vec<String> = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let red = 12;
    let green = 13;
    let blue = 14;
    let mut sum = 0;
    for line in lines {
        let base: Vec<&str> = line.split(": ").collect();
        let id = base[0].split(" ").collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();
        let sets = base[1].split("; ").collect::<Vec<&str>>();

        let mut ok = true;
        'outer: for set in sets {
            let colors = set.split(", ").collect::<Vec<&str>>();
            for color in colors {
                let mut split = color.split(" ");
                if let (Some(num_str), Some(color)) = (split.next(), split.next()) {
                    let num = num_str.parse::<i32>().unwrap();
                    match color {
                        "blue" if num > blue => ok = false,
                        "red" if num > red => ok = false,
                        "green" if num > green => ok = false,
                        _ => ok = true,
                    }
                    if ok == false {
                        break 'outer;
                    }
                }
            }
        }
        if ok {
            sum += id
        }
    }
    println!("{:?}", sum)
}

fn part2() {
    let filename = "input";
    let lines: Vec<String> = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut sum = 0;
    for line in lines {
        let base: Vec<&str> = line.split(": ").collect();
        let sets = base[1].split("; ").collect::<Vec<&str>>();
        let mut red_max = 0;
        let mut green_max = 0;
        let mut blue_max = 0;
        for set in sets {
            let colors = set.split(", ").collect::<Vec<&str>>();
            for color in colors {
                let mut split = color.split(" ");
                if let (Some(num_str), Some(color)) = (split.next(), split.next()) {
                    let num = num_str.parse::<i32>().unwrap();
                    match color {
                        "blue" => blue_max = max(blue_max, num),
                        "red" => red_max = max(red_max, num),
                        "green" => green_max = max(green_max, num),
                        _ => (),
                    }
                }
            }
        }
        sum += red_max * green_max * blue_max
    }
    println!("{:?}", sum)
}

fn main() {
    part1();
    part2();
}
