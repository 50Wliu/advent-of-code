use std::ops::Range;

pub fn part_1(contents: &str) -> Result<u64, String> {
    let lines = contents.lines().collect::<Vec<_>>();
    let iter = lines.iter().enumerate();
    let mut result = 0;
    for line in iter {
        let mut nums: Vec<(Range<usize>, u64)> = vec![];
        let mut current_num: String = String::new();
        for (i, char) in line.1.chars().enumerate() {
            if char.is_ascii_digit() {
                current_num += &char.to_string();
            } else if !current_num.is_empty() {
                nums.push((
                    i - current_num.len()..i,
                    current_num.parse::<u64>().map_err(|err| err.to_string())?,
                ));
                current_num.clear();
            }
        }

        if !current_num.is_empty() {
            nums.push((
                line.1.len() - current_num.len()..line.1.len(),
                current_num.parse::<u64>().map_err(|err| err.to_string())?,
            ));
            current_num.clear();
        }

        for (range, num) in nums.iter() {
            let adjacents =
                get_adjacents(&lines, line.0, range, &|c| !c.is_ascii_digit() && c != '.')
                    .ok_or("Could not get adjacents".to_string())?;
            if !adjacents.is_empty() {
                result += num;
            }
        }
    }
    Ok(result)
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    let lines = contents.lines().collect::<Vec<_>>();
    let iter = lines.iter().enumerate();
    let mut result = 0;
    for line in iter {
        let matches = line.1.match_indices('*');
        for (index, _) in matches {
            let adjacents =
                get_adjacents(&lines, line.0, &(index..index + 1), &|c| c.is_ascii_digit())
                    .ok_or("Could not get adjacents".to_string())?;
            let mut nums: Vec<u64> = vec![];
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
                            if char.is_ascii_digit() {
                                current_num += &char.to_string();
                            } else {
                                break;
                            }
                        }

                        current_num = current_num.chars().rev().collect();

                        for char in line_halves.1.chars() {
                            if char.is_ascii_digit() {
                                current_num += &char.to_string();
                            } else {
                                break;
                            }
                        }

                        nums.push(current_num.parse::<u64>().map_err(|err| err.to_string())?);
                    } else {
                        let mut current_num: String = String::new();
                        if adjacent.col < index {
                            for char in line.1[0..=adjacent.col].chars().rev() {
                                if char.is_ascii_digit() {
                                    current_num += &char.to_string();
                                } else {
                                    break;
                                }
                            }

                            current_num = current_num.chars().rev().collect();
                        } else {
                            for char in line.1[adjacent.col..].chars() {
                                if char.is_ascii_digit() {
                                    current_num += &char.to_string();
                                } else {
                                    break;
                                }
                            }
                        }

                        nums.push(current_num.parse::<u64>().map_err(|err| err.to_string())?);
                    }
                }
                result += nums.iter().product::<u64>()
            }
        }
    }
    Ok(result)
}

struct Point {
    row: usize,
    col: usize,
}

fn get_adjacents(
    board: &[&str],
    row: usize,
    range: &Range<usize>,
    matcher: &dyn Fn(char) -> bool,
) -> Option<Vec<Point>> {
    let mut results = vec![];
    let line = board.get(row)?;

    if range.start > 0 && line.chars().nth(range.start - 1).is_some_and(matcher) {
        results.push(Point {
            row,
            col: range.start - 1,
        });
    }

    if line.chars().nth(range.end).is_some_and(matcher) {
        results.push(Point {
            row,
            col: range.end,
        });
    }

    // Above
    if row > 0 {
        let above = board.get(row - 1)?;
        let start = if range.start == 0 { 0 } else { range.start - 1 };

        let end = if range.end == above.len() {
            above.len()
        } else {
            range.end + 1
        };

        let matches = above[start..end].match_indices(matcher);
        let mut last_i: Option<usize> = None;
        for (i, _) in matches {
            if last_i.is_none() || i != last_i? + 1 {
                results.push(Point {
                    row: row - 1,
                    col: i + start,
                });
            }
            last_i = Some(i);
        }
    }

    // Below
    if let Some(below) = board.get(row + 1) {
        let start = if range.start == 0 { 0 } else { range.start - 1 };

        let end = if range.end == below.len() {
            below.len()
        } else {
            range.end + 1
        };

        let matches = below[start..end].match_indices(matcher);
        let mut last_i: Option<usize> = None;
        for (i, _) in matches {
            if last_i.is_none() || i != last_i? + 1 {
                results.push(Point {
                    row: row + 1,
                    col: i + start,
                });
            }
            last_i = Some(i);
        }
    }
    Some(results)
}
