pub fn part_1(contents: &str) -> Result<u64, String> {
    let lines = contents.lines();
    let sequences = lines
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().map_err(|err| err.to_string()))
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(sequences.iter().try_fold(0, |acc, sequence| {
        Some(acc + get_next_prediction(sequence)?)
    }).ok_or("Boom")? as u64)
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

    Ok(sequences.iter().try_fold(0, |acc, sequence| {
        Some(acc + get_previous_prediction(sequence)?)
    }).ok_or("Boom")? as u64)
}

fn get_next_prediction(sequence: &[i64]) -> Option<i64> {
    if sequence.is_empty() || sequence.iter().all(|item| *item == 0) {
        return Some(0);
    }

    let mut next_sequence = vec![];

    for window in sequence.windows(2) {
        next_sequence.push(window[1] - window[0]);
    }

    Some(get_next_prediction(&next_sequence)? + sequence.last()?)
}

fn get_previous_prediction(sequence: &[i64]) -> Option<i64> {
    if sequence.is_empty() || sequence.iter().all(|item| *item == 0) {
        return Some(0);
    }

    let mut next_sequence = vec![];

    for window in sequence.windows(2) {
        next_sequence.push(window[1] - window[0]);
    }

    Some(sequence.first()? - get_previous_prediction(&next_sequence)?)
}
