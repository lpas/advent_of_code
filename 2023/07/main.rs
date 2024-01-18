use std::cmp::{Ordering, Reverse};
use std::convert::TryInto;
use std::fs;

#[derive(Debug)]
struct Hand {
    cards: [u8; 5],
    bid: u32,
    card_type: u8,
}

fn main() {
    let filename = "input";
    let part2 = true;
    let lines = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let mut hands: Vec<Hand> = lines
        .iter()
        .map(|line| {
            let data = line.split_ascii_whitespace().collect::<Vec<&str>>();
            let cards = parse_cards(&data[0], part2);
            let card_type = card_type(cards, part2);
            return Hand {
                cards,
                bid: data[1].parse::<u32>().unwrap(),
                card_type,
            };
        })
        .collect();

    hands.sort_by(|a, b| cmp_hands(a, b));

    let result: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u32 * hand.bid)
        .sum();

    println!("{result:?}");
}

fn parse_cards(str: &str, part2: bool) -> [u8; 5] {
    str.chars()
        .map(|c| match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' if !part2 => 11,
            'J' if part2 => 1,
            'T' => 10,
            _ => c.to_digit(10).unwrap() as u8,
        })
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap_or_else(|v: Vec<u8>| {
            panic!("Expected a Vec of length {} but it was {}", 5, v.len())
        })
}

fn card_type(cards: [u8; 5], part2: bool) -> u8 {
    let mut counts: Vec<u8> = vec![0; 15];
    for card in cards {
        counts[card as usize] += 1;
    }

    let joker_count = if part2 { counts[1] } else { 0 };

    if part2 {
        counts = counts.into_iter().skip(2).collect::<Vec<u8>>();
    }

    counts.sort_by_key(|w| Reverse(*w));

    let c1 = counts[0];
    let c2 = counts[1];

    if c1 == 5 - joker_count {
        return 6;
    }
    if c1 == 4 - joker_count {
        return 5;
    }
    if (joker_count == 0 && (c1 == 3 && c2 == 2))
        || (joker_count == 1 && ((c1 == 3 && c2 == 1) || (c1 == 2 && c2 == 2)))
        || (joker_count == 2 && ((c1 == 3 && c2 == 0) || (c1 == 2 && c2 == 1)))
        || (joker_count == 3 && ((c1 == 2 && c2 == 0) || (c1 == 1 && c2 == 1)))
        || (joker_count == 4 && (c1 == 1 && c2 == 0))
    {
        return 4;
    }

    if c1 == 3 - joker_count {
        return 3;
    }

    if (joker_count == 0 && (c1 == 2 && c2 == 2))
        || (joker_count == 1 && (c1 == 2 && c2 == 1))
        || (joker_count == 2 && ((c1 == 2) || (c1 == 1 && c2 == 1)))
    {
        return 2;
    }

    if (joker_count == 0 && (c1 == 2)) || (joker_count == 1 && (c1 == 1)) {
        return 1;
    }

    return 0;
}

fn cmp_hands(a: &Hand, b: &Hand) -> Ordering {
    if a.card_type != b.card_type {
        return a.card_type.cmp(&b.card_type);
    }
    for n in 0..5 {
        if a.cards[n] != b.cards[n] {
            return a.cards[n].cmp(&b.cards[n]);
        }
    }
    return Ordering::Equal;
}
