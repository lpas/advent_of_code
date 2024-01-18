use std::{
    collections::{HashMap, VecDeque},
    fs,
};

#[derive(Debug, PartialEq, Eq, Clone)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Boardcast,
    End,
}

#[derive(Debug, PartialEq, Eq)]
struct Module {
    name: String,
    destinations: Vec<String>,
    module_type: ModuleType,
    value: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Module2 {
    id: usize,
    name: String,
    destinations: Vec<usize>,
    module_type: ModuleType,
    value: bool,
    inputs: Vec<(usize, bool)>,
}

const BROADCASTER: &str = "broadcaster";

fn main() {
    let filename = "input";
    let lines = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let modules = ttr1(lines);

    let broadcaster_id = get_module_id(&modules, BROADCASTER);

    let mut modules_copy = modules.clone();
    const LOOPS: usize = 1000;
    let mut low = 0;
    let mut high = 0;
    for _ in 0..LOOPS {
        let r = cycle(broadcaster_id, &mut modules_copy, 0);
        high += r.0;
        low += r.1;
    }
    println!("part1: {low} * {high} = {}", low * high);

    // individuell solution names for nodes from .dot file
    let ids = vec!["dh", "dp", "bb", "qd"];
    let part2 = ids
        .iter()
        .map(|id| {
            let mut modules_copy = modules.clone();
            let dh_id = get_module_id(&modules_copy, id);
            let mut loops = 0;
            loop {
                loops += 1;
                let (_, _, v) = cycle(broadcaster_id, &mut modules_copy, dh_id);
                if v {
                    return loops;
                }
            }
        })
        .reduce(|a, b| lcm(a, b));

    println!("part2: {part2:?}");
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

fn get_module_id(modules: &Vec<Module2>, name: &str) -> usize {
    let broadcaster = modules.iter().find(|module| module.name == name).unwrap();

    let broadcaster_id = broadcaster.id.clone();
    broadcaster_id
}

fn cycle(broadcaster_id: usize, modules: &mut Vec<Module2>, check_id: usize) -> (u32, u32, bool) {
    let mut low = 0;
    let mut high = 0;
    let mut empty_reached = false;

    let mut queue: VecDeque<(usize, usize, bool)> = VecDeque::new();
    queue.push_back((0, broadcaster_id, false));
    while let Some((from, current_id, value)) = queue.pop_front() {
        let module = &mut modules[current_id];

        if current_id == 4 {
            // println!("{} -> {}, {}, {:?}", from, current_id, value, module.name);
        }
        if current_id == check_id && !value {
            empty_reached = true;
        }

        if value {
            high += 1;
        } else {
            low += 1;
        }

        // println!("{} -> {}, {}, {:?}", from, current_id, value, module.name);

        if let Some(new_value) = match module.module_type {
            ModuleType::Boardcast => Some(value),
            ModuleType::FlipFlop if value => None,
            ModuleType::FlipFlop => {
                module.value = !module.value;
                Some(module.value)
            }
            ModuleType::Conjunction => {
                let idx = module
                    .inputs
                    .iter()
                    .position(|&(id, _)| id == from)
                    .unwrap();
                module.inputs[idx].1 = value;
                Some(!module.inputs.iter().all(|&(_, v)| v))
            }
            ModuleType::End => None,
        } {
            for &dest in module.destinations.iter() {
                queue.push_back((current_id, dest, new_value));
            }
        }
    }

    (high, low, empty_reached)
}

fn ttr1(lines: Vec<String>) -> Vec<Module2> {
    let modules: Vec<(Module2, Vec<String>)> = lines
        .iter()
        .enumerate()
        .map(|(id, line)| {
            let (front, dest) = line.split_once("->").unwrap();
            let (module_type, name) = parse_front(front);
            let destinations = dest
                .split(",")
                .map(|item| item.trim().to_owned())
                .collect::<Vec<String>>();
            (
                Module2 {
                    id: id + 2,
                    name,
                    value: false,
                    module_type,
                    destinations: vec![],
                    inputs: vec![],
                },
                destinations,
            )
        })
        .collect();

    let mut m = vec![
        (
            Module2 {
                id: 0,
                name: "__DUMMY__".to_owned(),
                value: false,
                module_type: ModuleType::End,
                destinations: vec![],
                inputs: vec![],
            },
            vec![],
        ),
        (
            Module2 {
                id: 1,
                name: "rx".to_owned(),
                value: false,
                module_type: ModuleType::End,
                destinations: vec![],
                inputs: vec![],
            },
            vec![],
        ),
    ];
    m.extend(modules);

    let modules = m;

    let map: HashMap<String, usize> = modules
        .iter()
        .map(|(module, _)| (module.name.clone(), module.id))
        .collect();

    let mut modules: Vec<Module2> = modules
        .into_iter()
        .map(|(module, dests)| {
            let mut module = module.clone();
            module.destinations = dests
                .iter()
                .map(|str| {
                    if let Some(&id) = map.get(str) {
                        id
                    } else {
                        println!("{str}");
                        0
                    }
                })
                .collect();
            return module;
        })
        .collect();

    modules.clone().into_iter().for_each(|module| {
        for &dest in module.destinations.iter() {
            if modules[dest].module_type == ModuleType::Conjunction {
                modules[dest].inputs.push((module.id, false));
            }
        }
    });

    modules
}

#[allow(dead_code)]
fn get_modules(
    lines: Vec<String>,
) -> (
    HashMap<String, Module>,
    HashMap<String, HashMap<String, bool>>,
) {
    let modules: HashMap<String, Module> = lines
        .iter()
        .map(|line| {
            let (front, dest) = line.split_once("->").unwrap();
            let (module_type, name) = parse_front(front);
            let destinations = dest
                .split(",")
                .map(|item| item.trim().to_owned())
                .collect::<Vec<String>>();
            (
                name.clone(),
                Module {
                    name,
                    value: false,
                    destinations,
                    module_type,
                },
            )
        })
        .collect();

    let mut inputs: HashMap<String, HashMap<String, bool>> = HashMap::new();

    modules.values().for_each(|module| {
        for dest in module.destinations.iter() {
            let dest_module = modules.get(dest).unwrap();
            if dest_module.module_type == ModuleType::Conjunction {
                if let Some(map) = inputs.get_mut(&dest_module.name) {
                    map.insert(module.name.clone(), false);
                } else {
                    let mut map: HashMap<String, bool> = HashMap::new();
                    map.insert(module.name.clone(), false);
                    inputs.insert(dest.to_owned(), map);
                };
            };
        }
    });

    (modules, inputs)
}

fn parse_front(front: &str) -> (ModuleType, String) {
    let front = front.trim();

    let (module_type, name) = if front == "broadcaster" {
        (ModuleType::Boardcast, BROADCASTER.to_owned())
    } else {
        let mut chars = front.chars();
        (
            if chars.next().unwrap() == '%' {
                ModuleType::FlipFlop
            } else {
                ModuleType::Conjunction
            },
            chars.collect::<String>(),
        )
    };
    (module_type, name)
}
