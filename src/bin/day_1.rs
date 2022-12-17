use std::collections::BinaryHeap;
use std::fs;

fn main() {
    // Heap of Elves' Calories
    let mut elves_total_calories: BinaryHeap<i32> = BinaryHeap::new();

    // Read a file:
    let input_calories = fs::read_to_string("../../inputs/day_1/example1.txt").unwrap();
    // Will panic, file opening error unhandled

    let elves_calories = input_calories.split("\n\r");

    elves_calories.for_each(|elf_calories| {
        let elf_total_calories: i32 = elf_calories
            .split("\n")
            .map(|calorie: &str| calorie.parse::<i32>().unwrap())
            .sum();

        elves_total_calories.push(elf_total_calories);
    });

    assert_eq!(elves_total_calories.pop().unwrap_or(0), 24000);
}
