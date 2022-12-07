use std::collections::{HashSet, VecDeque};

pub fn part_1() -> usize {
    let contents = super::utilities::read_input(6);
    find_first_unique_sequence(contents, 4).expect("Expected a start-of-packet marker to exist")
}

pub fn part_2() -> usize {
    let contents = super::utilities::read_input(6);
    find_first_unique_sequence(contents, 14).expect("Expected a start-of-message marker to exist")
}

fn find_first_unique_sequence(contents: String, len: usize) -> Option<usize> {
    let mut search = contents.chars().take(len - 1).collect::<VecDeque<char>>();
    for (i, ch) in contents.chars().skip(len - 1).enumerate() {
        search.push_back(ch);
        let hashset = search.iter().collect::<HashSet<&char>>();
        if hashset.len() == len {
            return Some(i + len);
        }
        search.pop_front();
    }

    None
}
