use std::fs;

fn main() {
    let elf_pairs = fs::read_to_string("./inputs/day_4/example1.txt").unwrap();

    let mut overlapping_pairs = 0;

    elf_pairs.lines().for_each(|elf_pair| {
        let sections: Vec<&str> = elf_pair.split(",").collect();

        let elf_1_sections = Sections::new(sections[0]);
        let elf_2_sections = Sections::new(sections[1]);

        if sections_overlap(&elf_1_sections, &elf_2_sections) {
            overlapping_pairs += 1;
        }
    });

    println!("Overlapping pairs: {}", overlapping_pairs);
}

#[derive(Debug)]
struct Sections(i32, i32);

impl Sections {
    fn new(section_input: &str) -> Sections {
        let section_input: Vec<&str> = section_input.split("-").collect();

        let lowest_section = section_input[0].parse::<i32>().unwrap();

        let highest_section = section_input[1].parse::<i32>().unwrap();

        Sections(lowest_section, highest_section)
    }
}

fn sections_overlap(elf_1_sections: &Sections, elf_2_sections: &Sections) -> bool {
    elf_1_sections.0 >= elf_2_sections.0 && elf_1_sections.1 <= elf_2_sections.1
        || elf_2_sections.0 >= elf_1_sections.0 && elf_2_sections.1 <= elf_1_sections.1
}
