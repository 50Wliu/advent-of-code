pub fn part_1(contents: &String) -> u32 {
    contents.lines().fold(0, |acc, line| {
        let matches = line.matches(char::is_numeric).collect::<Vec<&str>>();
        acc +
            matches.first().unwrap().parse::<u32>().unwrap() * 10 +
            matches.last().unwrap().parse::<u32>().unwrap()
    })
}

pub fn part_2(contents: &String) -> u32 {
    contents.lines().fold(0, |acc, line| {
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

        let first = match_to_int(&min.1);
        let last = match_to_int(&max.1);
        acc + 10 * first + last
    })
}

fn match_to_int(m: &str) -> u32 {
  match m.chars().nth(0).unwrap().to_digit(10) {
    Some(digit) => digit,
    None => match m {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!()
      }
  }
}

fn line_to_matches(line: &str) -> Vec<(usize, &str)> {
    let mut matches = line.match_indices(char::is_numeric).collect::<Vec<_>>();
    matches.append(&mut line.match_indices("one").collect::<Vec<_>>());
    matches.append(&mut line.match_indices("two").collect::<Vec<_>>());
    matches.append(&mut line.match_indices("three").collect::<Vec<_>>());
    matches.append(&mut line.match_indices("four").collect::<Vec<_>>());
    matches.append(&mut line.match_indices("five").collect::<Vec<_>>());
    matches.append(&mut line.match_indices("six").collect::<Vec<_>>());
    matches.append(&mut line.match_indices("seven").collect::<Vec<_>>());
    matches.append(&mut line.match_indices("eight").collect::<Vec<_>>());
    matches.append(&mut line.match_indices("nine").collect::<Vec<_>>());
    matches
}