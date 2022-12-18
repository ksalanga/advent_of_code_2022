use std::{collections::HashSet, fs, str::Chars};

fn main() {
    let rucksacks = fs::read_to_string("../../inputs/day_3/example1.txt").unwrap();

    let mut priorities_sum = 0;

    rucksacks.lines().for_each(|rucksack| {
        let num_items = rucksack.chars().count();

        let compartment_1 = &rucksack[0..num_items / 2];
        let compartment_2 = &rucksack[num_items / 2..];

        let duplicate_item = find_duplicate_item(compartment_1.chars(), compartment_2.chars());

        priorities_sum += priority_score(duplicate_item);
    });

    println!("Priorities Sum: {}", priorities_sum);
}

fn find_duplicate_item(
    compartment_1: impl Iterator<Item = char>,
    compartment_2: impl Iterator<Item = char>,
) -> char {
    let mut items: HashSet<char> = HashSet::new();

    compartment_1.for_each(|item| {
        items.insert(item);
    });

    compartment_2
        .filter(|item| items.contains(item))
        .next()
        .unwrap_or_default()
}

fn priority_score(item: char) -> u32 {
    if item.is_lowercase() {
        let score = item as u32 - 'a' as u32 + 1;
        score
    } else {
        let score = item as u32 - 'A' as u32 + 27;
        score
    }
}
