use std::fs;

#[derive(Debug)]
struct CharPos {
    x: usize,
    y: usize,
    c: char,
}

#[derive(Debug)]
struct Digit {
    num: u32,
    surroundings: Vec<CharPos>,
}

fn main() {
    let filename = "input";
    let lines: Vec<Vec<char>> = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let lines_count = lines.len();
    let line_length = lines[0].len();

    let mut curr_digit = false;
    let mut digit: String = "".to_owned();

    let get_surroundings = |x: usize, y: usize, digit_length: usize| {
        let mut surrounding: Vec<CharPos> = vec![];

        let start = if x - digit_length > 0 {
            x - digit_length - 1
        } else {
            x - digit_length
        };
        let end = if x < line_length { x } else { x - 1 };
        // can add part of the digit but we don't care
        surrounding.push(CharPos {
            x: start,
            y,
            c: lines[y][start],
        });
        surrounding.push(CharPos {
            x: end,
            y,
            c: lines[y][end],
        });
        if y > 0 {
            for n in start..end + 1 {
                surrounding.push(CharPos {
                    x: n,
                    y: y - 1,
                    c: lines[y - 1][n],
                })
            }
        }
        if y + 1 < lines_count {
            for n in start..end + 1 {
                surrounding.push(CharPos {
                    x: n,
                    y: y + 1,
                    c: lines[y + 1][n],
                })
            }
        }
        return surrounding;
    };

    let mut digits: Vec<Digit> = vec![];

    for (y, line) in lines.iter().enumerate() {
        digit = "".to_owned();
        for (x, c) in line.iter().enumerate() {
            if c.is_digit(10) {
                digit.push(*c);
                curr_digit = true;
            } else if curr_digit {
                digits.push(Digit {
                    num: digit.parse::<u32>().unwrap(),
                    surroundings: get_surroundings(x, y, digit.len()),
                });
                digit = "".to_owned();
                curr_digit = false;
            }
        }
        if curr_digit {
            digits.push(Digit {
                num: digit.parse::<u32>().unwrap(),
                surroundings: get_surroundings(line_length, y, digit.len()),
            });
            curr_digit = false
        }
    }
    if curr_digit {
        digits.push(Digit {
            num: digit.parse::<u32>().unwrap(),
            surroundings: get_surroundings(line_length, lines_count, digit.len()),
        });
    }

    // part 1
    let mut sum = 0;
    for item in &digits {
        if item
            .surroundings
            .iter()
            .any(|c| !c.c.is_digit(10) && c.c != '.')
        {
            sum += item.num
        }
    }
    println!("{sum}");

    // part 2

    use std::collections::HashMap;
    #[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
    struct Pos {
        x: usize,
        y: usize,
    }

    let mut gears: HashMap<Pos, Vec<u32>> = HashMap::new();
    for item in &digits {
        for curr in item
            .surroundings
            .iter()
            .filter(|c| c.c == '*')
            .collect::<Vec<&CharPos>>()
        {
            let pos = Pos {
                x: curr.x,
                y: curr.y,
            };
            if !gears.contains_key(&pos) {
                gears.insert(pos, Vec::new());
            }
            gears.get_mut(&pos).unwrap().push(item.num);
        }
    }

    let sum2: u32 = gears
        .iter()
        .map(|(_key, val)| if val.len() == 2 { val[0] * val[1] } else { 0 })
        .sum();

    println!("{sum2}")
}
