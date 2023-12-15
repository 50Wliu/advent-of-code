pub fn part_1(contents: &str) -> Result<u64, String> {
    let lines = contents.lines();
    let sequences = lines
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().map_err(|err| err.to_string()))
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(sequences
        .iter()
        .fold(0, |acc, sequence| acc + get_next_prediction(sequence)) as u64)
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    let lines = contents.lines();
    let sequences = lines
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().map_err(|err| err.to_string()))
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(sequences
        .iter()
        .fold(0, |acc, sequence| acc + get_previous_prediction(sequence)) as u64)
}

fn get_next_prediction(sequence: &[i64]) -> i64 {
    if sequence.iter().all(|item| *item == 0) {
        return 0;
    }

    let next_sequence = sequence
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<_>>();

    get_next_prediction(&next_sequence) + sequence[sequence.len() - 1]
}

fn get_previous_prediction(sequence: &[i64]) -> i64 {
    if sequence.iter().all(|item| *item == 0) {
        return 0;
    }

    let next_sequence = sequence
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<_>>();

    sequence[0] - get_previous_prediction(&next_sequence)
}
