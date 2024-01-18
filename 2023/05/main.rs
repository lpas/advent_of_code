use std::{cmp, fmt::Debug, fs};

fn main() {
    let filename = "input";
    let mut binding = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let mut lines = binding.iter_mut();

    let seeds = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|item| item.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    #[derive(Debug, Clone, Copy)]
    struct Range {
        source: u64,
        dest: u64,
        length: u64,
    }
    // skip empty line & first label
    lines.next();
    lines.next();

    let mut index = 0;
    let mut maps: Vec<Vec<Range>> = vec![vec![]; 7];
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if line.contains(":") {
            index += 1;
            continue;
        }

        if let [dest, source, length] = line
            .split_whitespace()
            .map(|item| item.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()[..]
        {
            maps[index].push(Range {
                source,
                dest,
                length,
            });
        }
    }

    for item in maps.iter_mut() {
        item.sort_by(|a, b| a.source.cmp(&b.source))
    }

    fn find_loc(mut num: u64, maps: &Vec<Vec<Range>>, index: usize) -> u64 {
        for item in &maps[index] {
            if item.source <= num && item.source + item.length - 1 >= num {
                num = item.dest + num - item.source;
                break;
            }
        }
        if index < 6 {
            return find_loc(num, maps, index + 1);
        }
        return num;
    }

    let locs: Vec<u64> = seeds.iter().map(|seed| find_loc(*seed, &maps, 0)).collect();
    let result = locs.iter().min();
    println!("{result:?}");

    fn overlap(a: (u64, u64), b: (u64, u64)) -> bool {
        !((a.0 < b.0 && a.1 < b.0) || (a.0 > b.1 && a.1 > b.1))
    }

    fn find_loc_range(
        range: Vec<(u64, u64)>,
        maps: &Vec<Vec<Range>>,
        index: usize,
    ) -> Vec<(u64, u64)> {
        let mut result: Vec<(u64, u64)> = vec![];
        'outer: for (mut start, end) in range {
            for item in &maps[index] {
                let item_start = item.source;
                let item_end = item.source + item.length - 1;
                if overlap((start, end), (item_start, item_end)) {
                    if start < item.source {
                        result.push((start, item.source - 1))
                    }
                    result.push((
                        item.dest + start - item.source,
                        item.dest + cmp::min(end, item_end) - item.source,
                    ));
                    if end > item_end {
                        start = item_end + 1
                    } else {
                        continue 'outer;
                    }
                }
            }
            result.push((start, end));
        }
        if index < 6 {
            return find_loc_range(result, maps, index + 1);
        }
        return result;
    }

    let result2 = seeds
        .chunks(2)
        .map(|item| {
            find_loc_range(vec![(item[0], item[0] + item[1])], &maps, 0)
                .iter()
                .map(|item| item.0)
                .min()
                .unwrap()
        })
        .min();

    println!("{result2:?}");
}
