use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Rule {
    category: char,
    compare: char,
    value: i32,
    dest: String,
}
type Rules = Vec<Rule>;
#[derive(Debug, Copy, Clone)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

fn main() {
    let filename = "input";
    let lines = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let mut workflows: HashMap<String, Rules> = HashMap::new();

    let mut lines = lines.iter();
    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }
        let (key, rules) = parse_workflow_line(line);
        workflows.insert(key, rules);
    }

    let mut sum = 0;
    while let Some(line) = lines.next() {
        let part = parse_part(line);
        let mut key = "in";
        loop {
            let rules = workflows.get(key).unwrap();
            key = next_dest(rules, &part);
            if key == "A" || key == "R" {
                break;
            }
        }
        if key == "A" {
            sum += part.x + part.m + part.a + part.s;
        }
    }
    println!("part1: {sum}");

    let mut queue = vec![(
        "in",
        Part {
            x: 1,
            m: 1,
            a: 1,
            s: 1,
        },
        Part {
            x: 4000,
            m: 4000,
            a: 4000,
            s: 4000,
        },
        vec![],
    )];

    let mut sum = 0;
    while let Some((dest, mut min, mut max, mut path)) = queue.pop() {
        path.push(dest);
        if dest == "A" {
            let x = (max.x - min.x) as u64 + 1;
            let m = (max.m - min.m) as u64 + 1;
            let s = (max.s - min.s) as u64 + 1;
            let a = (max.a - min.a) as u64 + 1;
            let value = x * m * a * s;
            sum += value;
            continue;
        } else if dest == "R" {
            continue;
        }

        let rules = workflows.get(dest).unwrap();
        for rule in rules {
            if rule.category == '_' {
                queue.push((&rule.dest, min, max, path.clone()));
            } else {
                if rule.compare == '<' {
                    queue.push((
                        &rule.dest,
                        min,
                        set_value(rule.category, &max, rule.value - 1),
                        path.clone(),
                    ));
                    min = set_value(rule.category, &min, rule.value);
                } else {
                    queue.push((
                        &rule.dest,
                        set_value(rule.category, &min, rule.value + 1),
                        max,
                        path.clone(),
                    ));
                    max = set_value(rule.category, &max, rule.value);
                }
            }
        }
    }

    println!("part2: {}", sum);
}

fn set_value(category: char, part: &Part, value: i32) -> Part {
    let mut new_part = part.clone();
    match category {
        'x' => new_part.x = value,
        'm' => new_part.m = value,
        'a' => new_part.a = value,
        's' => new_part.s = value,
        _ => panic!("not a valid category"),
    }

    return new_part;
}

fn next_dest<'a>(rules: &'a Rules, part: &'a Part) -> &'a str {
    for rule in rules {
        if rule.category == '_' {
            return &rule.dest;
        }
        let part_value = match rule.category {
            'm' => part.m,
            'a' => part.a,
            'x' => part.x,
            's' => part.s,
            _ => panic!("not valid category"),
        };

        if (rule.compare == '>' && part_value > rule.value)
            || (rule.compare == '<' && part_value < rule.value)
        {
            return &rule.dest;
        }
    }
    panic!("should never happen");
}

fn parse_part(line: &String) -> Part {
    let mut part = Part {
        x: 0,
        m: 0,
        a: 0,
        s: 0,
    };

    for item in line[1..line.len() - 1].split(",") {
        let mut item = item.chars();
        let category = item.next().unwrap();
        item.next();
        let value = item.collect::<String>().parse::<i32>().unwrap();
        match category {
            'x' => part.x = value,
            'm' => part.m = value,
            'a' => part.a = value,
            's' => part.s = value,
            _ => (),
        };
    }
    part
}

fn parse_workflow_line(line: &String) -> (String, Vec<Rule>) {
    let (key, rest) = line.split_once("{").unwrap();
    let rest = rest.trim_end_matches("}");
    let rules = rest
        .split(",")
        .into_iter()
        .map(|item| {
            if !item.contains(":") {
                return Rule {
                    category: '_',
                    compare: '_',
                    value: 0,
                    dest: item.to_owned(),
                };
            }
            let (front, dest) = item.split_once(":").unwrap();
            let dest = dest.to_owned();
            let mut front = front.chars();
            let category = front.next().unwrap();
            let compare = front.next().unwrap();
            let value = front.collect::<String>().parse::<i32>().unwrap();
            Rule {
                category,
                compare,
                value,
                dest,
            }
        })
        .collect::<Rules>();
    (key.to_owned(), rules)
}
