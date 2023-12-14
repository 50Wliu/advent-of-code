use std::ops::Range;

#[derive(Debug)]
struct Mapping {
    offset: i64,
    source: Range<u64>,
    dest: Range<u64>,
}

pub fn part_1(contents: &str) -> Result<u64, String> {
    let mut lines = contents.lines();
    let seeds = lines
        .next()
        .ok_or("Missing seeds")?
        .strip_prefix("seeds: ")
        .ok_or("Missing `seeds: ` prefix")?
        .split_whitespace()
        .map(|seed| seed.parse::<u64>().map_err(|err| err.to_string()))
        .collect::<Result<Vec<_>, _>>()?;

    let mappings = parse_mappings(lines)?;
    seeds
        .into_iter()
        .map(|seed| seed_to_location(seed, &mappings))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .min()
        .ok_or("No minimum".to_string())
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    let mut lines = contents.lines();
    let mut seeds = lines
        .next()
        .ok_or("Missing seeds")?
        .strip_prefix("seeds: ")
        .ok_or("Missing `seeds: ` prefix")?
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
    for mapping in mappings.into_iter() {
        let mut new_seeds = vec![];
        for seed_range in seeds {
            new_seeds.append(&mut map_seed_range(seed_range, mapping.iter())?);
        }
        seeds = new_seeds;
    }

    Ok(seeds
        .iter()
        .min_by_key(|seed_range| seed_range.start)
        .ok_or("No minimum".to_string())?
        .start)
}

fn parse_mappings<'a>(lines: impl Iterator<Item = &'a str>) -> Result<Vec<Vec<Mapping>>, String> {
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

        mappings.last_mut().ok_or("No last element")?.push(Mapping {
            offset: destination_start as i64 - source_start as i64,
            source: source_start..source_start + range,
            dest: destination_start..destination_start + range,
        });
    }

    Ok(mappings)
}

fn seed_to_location(seed: u64, mappings: &[Vec<Mapping>]) -> Result<u64, String> {
    let mut location = seed;
    for mapping in mappings {
        for map in mapping {
            if map.source.contains(&location) {
                location = location.checked_add_signed(map.offset).ok_or(format!(
                    "Overflow while adding {} to {}",
                    location, map.offset
                ))?;
                break;
            }
        }
    }
    Ok(location)
}

fn map_seed_range<'a>(
    range: Range<u64>,
    mut mappings: impl Iterator<Item = &'a Mapping>,
) -> Result<Vec<Range<u64>>, String> {
    let mut result = vec![];
    if let Some(mapping) = mappings.next() {
        if range.end > mapping.source.start
            && range.end < mapping.source.end
            && range.start <= mapping.source.start
        {
            // Case 1: Upper bound
            // seeds: [ 0..10 ]
            // 0..15 -> 20..35 (+20)
            // [20..30]
            if range.start != mapping.source.start {
                result.append(&mut map_seed_range(
                    range.start..mapping.source.start,
                    mappings,
                )?);
            }
            result.push(
                mapping.dest.start
                    ..range.end.checked_add_signed(mapping.offset).ok_or(format!(
                        "Case 1 overflow while processing range {:?} and mapping {:?}",
                        range, mapping
                    ))?,
            );
        } else if range.start > mapping.source.start
            && range.start < mapping.source.end
            && range.end >= mapping.source.end
        {
            // Case 2: Lower bound
            if mapping.source.end != range.end {
                result.append(&mut map_seed_range(
                    mapping.source.end..range.end,
                    mappings,
                )?);
            }
            result.push(
                range
                    .start
                    .checked_add_signed(mapping.offset)
                    .ok_or(format!(
                        "Case 2 overflow while processing range {:?} and mapping {:?}",
                        range, mapping
                    ))?..mapping.dest.end,
            );
        } else if range.start <= mapping.source.start && range.end >= mapping.source.end {
            // Case 3: Sub-range
            // seeds: [ 0..10 ]
            // 0..7 -> 20..27 (+20)
            // [20..27] [7..10]
            let mappings = mappings.collect::<Vec<_>>();
            if range.start != mapping.source.start {
                result.append(&mut map_seed_range(
                    range.start..mapping.source.start,
                    mappings.clone().into_iter(),
                )?);
            }
            result.push(mapping.dest.clone());
            if mapping.source.end != range.end {
                result.append(&mut map_seed_range(
                    mapping.source.end..range.end,
                    mappings.into_iter(),
                )?);
            }
        } else if range.start > mapping.source.start && range.end < mapping.source.end {
            // Case 4: Super-range
            result.push(
                range
                    .start
                    .checked_add_signed(mapping.offset)
                    .ok_or(format!(
                        "Case 4 overflow while processing range {:?} and mapping {:?}",
                        range, mapping
                    ))?
                    ..range.end.checked_add_signed(mapping.offset).ok_or(format!(
                        "Case 4 overflow while processing range {:?} and mapping {:?}",
                        range, mapping
                    ))?,
            )
        } else {
            // Case 5: No overlap
            result.append(&mut map_seed_range(range, mappings)?);
        }
    } else {
        result.push(range);
    }

    Ok(result)
}
