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
                nums.push((i - current_num.len()..i, current_num.parse::<u32>().unwrap()));
                current_num.clear();
            }
        }

        if !current_num.is_empty() {
            nums.push((line.1.len() - current_num.len()..line.1.len(), current_num.parse::<u32>().unwrap()));
            current_num.clear();
        }

        for (range, num) in nums.iter() {
            if !get_adjacents(&lines, line.0, range, &|c| !c.is_digit(10) && c != '.').is_empty() {
                result += num;
            }
        }
    }
    result
}

pub fn part_2() -> u32 {
    let contents = utilities::read_input(3);
    let lines = contents.lines().collect::<Vec<_>>();
    let mut iter = lines.iter().enumerate();
    let mut result: u32 = 0;
    while let Some(line) = iter.next() {
        let matches = line.1.match_indices('*');
        for (index, _) in matches {
            let adjacents = get_adjacents(&lines, line.0, &(index..index + 1), &|c| c.is_digit(10));
            let mut nums: Vec<u32> = vec![];
            if adjacents.len() == 2 {
                for adjacent in adjacents {
                    if adjacent.row != line.0 {
                        let split_col = if adjacent.col < index {
                            adjacent.col + 1
                        } else {
                            adjacent.col
                        };
                        let line_halves = lines[adjacent.row].split_at(split_col);
                        let mut current_num: String = String::new();
                        for char in line_halves.0.chars().rev() {
                            if char.is_digit(10) {
                                current_num += &char.to_string();
                            } else {
                                break;
                            }
                        }

                        current_num = current_num.chars().rev().collect();

                        for char in line_halves.1.chars() {
                            if char.is_digit(10) {
                                current_num += &char.to_string();
                            } else {
                                break;
                            }
                        }

                        nums.push(current_num.parse::<u32>().unwrap());
                    } else {
                        let mut current_num: String = String::new();
                        if adjacent.col < index {
                            for char in line.1[0..=adjacent.col].chars().rev() {
                                if char.is_digit(10) {
                                    current_num += &char.to_string();
                                } else {
                                    break;
                                }
                            }

                            current_num = current_num.chars().rev().collect();
                        } else {
                            for char in line.1[adjacent.col..].chars() {
                                if char.is_digit(10) {
                                    current_num += &char.to_string();
                                } else {
                                    break;
                                }
                            }
                        }

                        nums.push(current_num.parse::<u32>().unwrap());
                    }
                }
                result += nums.iter().product::<u32>()
            }
        }
    }
    result
}

struct Point {
    row: usize,
    col: usize, 
}

fn get_adjacents(board: &Vec<&str>, row: usize, range: &Range<usize>, matcher: &dyn Fn(char) -> bool) -> Vec<Point> {
    let mut results = vec![];
    let line = board.get(row).unwrap();

    if range.start > 0 && line.chars().nth(range.start - 1).is_some_and(matcher) {
        results.push(Point {row, col: range.start - 1});
    }

    if line.chars().nth(range.end).is_some_and(matcher) {
        results.push(Point {row, col: range.end});
    }

    // Above
    if row > 0 {
        let above = board.get(row - 1).unwrap();
        let start = if range.start == 0 {
            0
        } else {
            range.start - 1
        };
            
        let end = if range.end == above.len() {
            above.len()
        } else {
            range.end + 1
        };

        let matches = above[start..end].match_indices(matcher);
        let mut last_i: Option<usize> = None;
        for (i, _) in matches {
            if last_i.is_none() || i != last_i.unwrap() + 1 {
                results.push(Point{row: row - 1, col: i + start});
            }
            last_i = Some(i);
        }
    }

    // Below
    if let Some(below) = board.get(row + 1) {
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

        let matches = below[start..end].match_indices(matcher);
        let mut last_i: Option<usize> = None;
        for (i, _) in matches {
            if last_i.is_none() || i != last_i.unwrap() + 1 {
                results.push(Point {row: row + 1, col: i + start});
            }
            last_i = Some(i);
        }
    }
    results
}
