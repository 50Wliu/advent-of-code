pub fn part_1(contents: &str) -> Result<u32, String> {
    contents.lines().try_fold(0, |acc, line| {
        let matches = line.matches(char::is_numeric).collect::<Vec<_>>();
        let first_digit = matches
            .first()
            .ok_or("No first match".to_string())?
            .parse::<u32>()
            .map_err(|err| err.to_string())?;
        let last_digit = matches
            .last()
            .ok_or("No last match".to_string())?
            .parse::<u32>()
            .map_err(|err| err.to_string())?;
        Ok(acc + first_digit * 10 + last_digit)
    })
}

pub fn part_2(contents: &str) -> Result<u32, String> {
    contents.lines().try_fold(0, |acc, line| {
        let matches = line_to_matches(line);

        let max = matches
            .iter()
            .fold((std::usize::MIN, ""), |acc, (i, value)| {
                if i >= &acc.0 {
                    (*i, value)
                } else {
                    acc
                }
            });
        let min = matches
            .iter()
            .fold((std::usize::MAX, ""), |acc, (i, value)| {
                if i <= &acc.0 {
                    (*i, value)
                } else {
                    acc
                }
            });

        let first = match_to_int(&min.1).ok_or("No first match".to_string())?;
        let last = match_to_int(&max.1).ok_or("No last match".to_string())?;
        Ok(acc + 10 * first + last)
    })
}

fn match_to_int(m: &str) -> Option<u32> {
    match m.chars().nth(0)?.to_digit(10) {
        Some(digit) => Some(digit),
        None => match m {
            "one" => Some(1),
            "two" => Some(2),
            "three" => Some(3),
            "four" => Some(4),
            "five" => Some(5),
            "six" => Some(6),
            "seven" => Some(7),
            "eight" => Some(8),
            "nine" => Some(9),
            _ => None,
        },
    }
}

fn line_to_matches(line: &str) -> Vec<(usize, &str)> {
    line.match_indices(char::is_numeric)
        .chain(line.match_indices("one"))
        .chain(line.match_indices("two"))
        .chain(line.match_indices("three"))
        .chain(line.match_indices("four"))
        .chain(line.match_indices("five"))
        .chain(line.match_indices("six"))
        .chain(line.match_indices("seven"))
        .chain(line.match_indices("eight"))
        .chain(line.match_indices("nine"))
        .collect()
}
