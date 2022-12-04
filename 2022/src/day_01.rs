pub fn part_1() -> u32 {
    let elf_calories = calculate_calories_per_elf();
    *elf_calories.first().expect("Vector should be non-empty")
}

pub fn part_2() -> u32 {
    let elf_calories = calculate_calories_per_elf();
    elf_calories[0..3].iter().sum::<u32>()
}

fn calculate_calories_per_elf() -> Vec<u32> {
    let contents = super::utilities::read_input(1);
    let mut calories_by_elf = contents.lines().fold(vec![0], |mut accum, line| {
        if line.is_empty() {
            accum.push(0);
        } else {
            let calories = line.parse::<u32>().expect("Line should be a number");
            let last = accum.last_mut().expect("Vector should be non-empty");
            *last += calories;
        }

        accum
    });

    calories_by_elf.sort_by(|a, b| b.cmp(a));

    calories_by_elf
}
