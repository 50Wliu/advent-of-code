pub fn part_1(contents: &str) -> Result<u64, String> {
    let mut lines = contents.lines();
    let times = lines
        .next()
        .ok_or("Missing times")?
        .strip_prefix("Time:")
        .ok_or("Missing `Time:` prefix")?
        .split_whitespace()
        .map(|time| time.parse::<u64>().map_err(|err| err.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    let distances = lines
        .next()
        .ok_or("Missing distances")?
        .strip_prefix("Distance:")
        .ok_or("Missing `Distance:` prefix")?
        .split_whitespace()
        .map(|distance| distance.parse::<u64>().map_err(|err| err.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    let races = times.into_iter().zip(distances);

    Ok(races.map(|(time, distance)| {
        (0..time).fold(0, |acc, charge_time| {
            if (time - charge_time) * charge_time > distance {
                acc + 1
            } else {
                acc
            }
        })
    }).product())
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    let mut lines = contents.lines();
    let time = lines
        .next()
        .ok_or("Missing times")?
        .strip_prefix("Time:")
        .ok_or("Missing `Time:` prefix")?
        .replace(' ', "")
        .parse::<u64>().map_err(|err| err.to_string())?;
    let distance = lines
        .next()
        .ok_or("Missing distances")?
        .strip_prefix("Distance:")
        .ok_or("Missing `Distance:` prefix")?
        .replace(' ', "")
        .parse::<u64>().map_err(|err| err.to_string())?;

    Ok((0..time).fold(0, |acc, charge_time| {
        if (time - charge_time) * charge_time > distance {
            acc + 1
        } else {
            acc
        }
    }))
}
