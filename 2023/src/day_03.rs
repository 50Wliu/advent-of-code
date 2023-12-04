use crate::utilities;
use std::ops::Range;

pub fn part_1() -> u32 {
    let contents = utilities::read_input(3);
    let lines = contents.lines().collect::<Vec<_>>();
    let mut iter = lines.iter().enumerate();
    let mut result: u32 = 0;
    while let Some(line) = iter.next() {
        let mut nums: Vec<(Range<usize>, u32)> = vec![];
        let mut current_num: String = String::new();
        for (i, char) in line.1.chars().enumerate() {
            if char.is_digit(10) {
                current_num += &char.to_string();
            } else if !current_num.is_empty() {
                nums.push((Range {start: i - current_num.len(), end: i}, current_num.parse::<u32>().unwrap()));
                current_num.clear();
            }
        }

        if !current_num.is_empty() {
            nums.push((Range {start: line.1.len() - current_num.len(), end: line.1.len()}, current_num.parse::<u32>().unwrap()));
            current_num.clear();
        }

        for (range, num) in nums.iter() {
            // Front
            if range.start > 0 && line.1.chars().nth(range.start - 1).is_some_and(|c| c != '.') {
                result += num;
                continue;
            }
            // Back
            if line.1.chars().nth(range.end).is_some_and(|c| c != '.') {
                result += num;
                continue;
            }

            // Above
            if line.0 != 0 {
                let below = lines[line.0 - 1];
                let start = if range.start == 0 {
                    0
                } else {
                    range.start - 1
                };
                    
                let end = if range.end == below.len() {
                    below.len()
                } else {
                    range.end + 1
                };

                if below[start..end].contains(|c: char| {
                    !c.is_digit(10) && c != '.'
                }) {
                    result += num;
                    continue;
                }
            }

            // Below
            if lines.get(line.0 + 1).is_some_and(|below| {
                let start = if range.start == 0 {
                    0
                } else {
                    range.start - 1
                };
                    
                let end = if range.end == below.len() {
                    below.len()
                } else {
                    range.end + 1
                };

                below[start..end].contains(|c: char| {
                    !c.is_digit(10) && c != '.'
                })
            }) {
                result += num;
                continue;
            }
        }

    }
    result
}

pub fn part_2() -> u32 {
    let _contents = utilities::read_input(3);
    0
}
