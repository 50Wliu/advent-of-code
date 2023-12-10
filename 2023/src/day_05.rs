use std::ops::Range;
use std::str::Lines;

struct Mapping {
    offset: i64,
    range: Range<u64>,
}

pub fn part_1(contents: &str) -> Result<u64, String> {
    let mut lines = contents.lines();
    let seeds = lines
        .next()
        .ok_or("Missing seeds".to_string())?
        .strip_prefix("seeds: ")
        .ok_or("Missing `seeds: ` prefix".to_string())?
        .split_whitespace()
        .map(|seed| seed.parse::<u64>().map_err(|err| err.to_string()))
        .collect::<Result<Vec<_>, _>>()?;

    let mappings = parse_mappings(lines)?;
    seeds
        .into_iter()
        .map(|seed| seed_to_location(seed, &mappings))
        .min()
        .ok_or("No minimum".to_string())
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    let mut lines = contents.lines();
    let seeds = lines
        .next()
        .ok_or("Missing seeds".to_string())?
        .strip_prefix("seeds: ")
        .ok_or("Missing `seeds: ` prefix".to_string())?
        .split_whitespace()
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .map(|seed| -> Result<_, String> {
            let start = seed[0].parse::<u64>().map_err(|err| err.to_string())?;
            let range = seed[1].parse::<u64>().map_err(|err| err.to_string())?;
            Ok(start..start + range)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mappings = parse_mappings(lines)?;
    Ok(seeds.into_iter().fold(std::u64::MAX, |acc, range| {
        let min = range.fold(std::u64::MAX, |acc, seed| {
            let location = seed_to_location(seed, &mappings);
            if location < acc {
                location
            } else {
                acc
            }
        });

        if min < acc {
            min
        } else {
            acc
        }
    }))
}

fn parse_mappings(lines: Lines) -> Result<Vec<Vec<Mapping>>, String> {
    let mut mappings = vec![];
    for line in lines {
        if line.is_empty() {
            continue;
        }

        if line.starts_with(char::is_alphabetic) {
            mappings.push(vec![]);
            continue;
        }

        let mut segments = line.split_whitespace();
        let destination_start = segments
            .next()
            .ok_or("Missing destination start")?
            .parse::<u64>()
            .map_err(|err| err.to_string())?;
        let source_start = segments
            .next()
            .ok_or("Missing source start")?
            .parse::<u64>()
            .map_err(|err| err.to_string())?;
        let range = segments
            .next()
            .ok_or("Missing range")?
            .parse::<u64>()
            .map_err(|err| err.to_string())?;

        mappings
            .last_mut()
            .ok_or("No last element".to_string())?
            .push(Mapping {
                offset: destination_start as i64 - source_start as i64,
                range: source_start..source_start + range,
            });
    }

    Ok(mappings)
}

fn seed_to_location(seed: u64, mappings: &[Vec<Mapping>]) -> u64 {
    let mut location = seed as i64;
    for mapping in mappings.iter() {
        for map in mapping.iter() {
            if map.range.contains(&(location as u64)) {
                location += map.offset;
                break;
            }
        }
    }
    location as u64
}
