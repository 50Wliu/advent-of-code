pub fn part_1(contents: &str) -> Result<u32, String> {
    contents.lines().fold(Ok(0), |acc, line| {
        match acc {
            Ok(acc) => {
                let matches = line.matches(char::is_numeric).collect::<Vec<&str>>();
                Ok(acc +
                    matches
                        .first()
                        .ok_or("No first match".to_string())?
                        .parse::<u32>()
                        .map_err(|err| err.to_string())? * 10 +
                    matches
                        .last()
                        .ok_or("No last match".to_string())?
                        .parse::<u32>()
                        .map_err(|err| err.to_string())?)
            }
            Err(err) => Err(err),
        }
    })
}

pub fn part_2(contents: &str) -> Result<u32, String> {
    contents.lines().fold(Ok(0), |acc, line| {
        match acc {
            Ok(acc) => {
                let matches = line_to_matches(line);

                let max = matches.iter().fold((std::usize::MIN, ""), |acc, (i, value)| {
                    if i >= &acc.0 {
                        (*i, value)
                    } else {
                        acc
                    }
                });
                let min = matches.iter().fold((std::usize::MAX, ""), |acc, (i, value)| {
                    if i <= &acc.0 {
                        (*i, value)
                    } else {
                        acc
                    }
                });

                let first = match_to_int(&min.1).ok_or("No first match".to_string())?;
                let last = match_to_int(&max.1).ok_or("No last match".to_string())?;
                Ok(acc + 10 * first + last)
            }
            Err(err) => Err(err),
        }
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
        _ => None
    }
  }
}

fn line_to_matches(line: &str) -> Vec<(usize, &str)> {
    let mut matches: Vec<_> = line.match_indices(char::is_numeric).collect();
    matches.append(&mut line.match_indices("one").collect());
    matches.append(&mut line.match_indices("two").collect());
    matches.append(&mut line.match_indices("three").collect());
    matches.append(&mut line.match_indices("four").collect());
    matches.append(&mut line.match_indices("five").collect());
    matches.append(&mut line.match_indices("six").collect());
    matches.append(&mut line.match_indices("seven").collect());
    matches.append(&mut line.match_indices("eight").collect());
    matches.append(&mut line.match_indices("nine").collect());
    matches
}
