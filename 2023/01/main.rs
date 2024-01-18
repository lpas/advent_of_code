use std::fs;

fn main() {
    let filename = "input";
    let lines: Vec<String> = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let mut sum = 0;

    for line in lines {
        let mut digit1 = '-';
        let mut digit1_pos = -1;
        let mut digit2 = '-';
        let mut digit2_pos = -1;
        let ((d1, pos1), (d2, pos2)) = get_by_string(line.as_str());

        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if digit1 == '-' {
                    digit1 = c;
                    digit1_pos = i as i32
                }
                digit2 = c;
                digit2_pos = i as i32
            }
        }
        if digit1 == '-' || digit1_pos == -1 || (digit1_pos > pos1 && pos1 != -1) {
            digit1 = d1
        }
        if digit2 == '-' || digit2_pos == -1 || (digit2_pos < pos2 && pos2 != -1) {
            digit2 = d2
        }

        let num = format!("{}{}", digit1, digit2).parse::<u32>().unwrap();
        sum += num;
    }
    println!("{}", sum)
}

fn get_by_string(line: &str) -> ((char, i32), (char, i32)) {
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut d1 = '-';
    let mut d2 = '-';
    let mut pos1: i32 = -1;
    let mut pos2: i32 = -1;
    for (i, digit) in digits.iter().enumerate() {
        if let Some(n) = line.find(digit) {
            if pos1 == -1 || (n as i32) < pos1 {
                pos1 = n as i32;
                d1 = char::from_digit(i as u32 + 1, 10).unwrap()
            }
        }
        if let Some(n) = line.rfind(digit) {
            if pos2 == -1 || (n as i32) > pos2 {
                pos2 = n as i32;
                d2 = char::from_digit(i as u32 + 1, 10).unwrap()
            }
        }
    }
    return ((d1, pos1), (d2, pos2));
}
