pub fn part_1() -> u32 {
    let contents = super::utilities::read_input(3);
    contents.lines().fold(0, |accum, rucksack| {
        if rucksack.len() % 2 != 0 {
            panic!("Rucksack contents should be evenly divided into two compartments");
        }

        let (left, right) = rucksack.split_at(rucksack.len() / 2);
        let offending_item = left.chars().find(|&char| right.contains(char))
            .expect("Both compartments should share an item");
        accum + calculate_priority(offending_item)
    })
}

pub fn part_2() -> u32 {
    let contents = super::utilities::read_input(3);
    let mut iter = contents.lines();
    let mut accum = 0;
    while let (Some(sack_1), Some(sack_2), Some(sack_3)) = (iter.next(), iter.next(), iter.next()) {
        let badge = sack_1.chars().find(|&char| sack_2.contains(char) && sack_3.contains(char))
            .expect("All three rucksacks should have a badge");

        accum += calculate_priority(badge)
    }

    accum
}

fn calculate_priority(item: char) -> u32 {
    if item.is_ascii_lowercase() {
        // 'a' = ascii 97
        item as u32 - 96
    } else {
        // 'A' = ascii 65, add 26 to get priorities starting at 27
        item as u32 - 64 + 26
    }
}
