use std::collections::HashMap;

pub fn part_1(contents: &str) -> Result<u64, String> {
    let mut lines = contents.lines();

    let directions = lines.next().ok_or("Missing directions")?.chars().cycle();

    let network = build_network(lines.skip(1))?;

    let mut current_node = "AAA";
    let mut steps = 0;
    for direction in directions {
        steps += 1;
        let destinations = network
            .get(current_node)
            .ok_or(format!("Missing node {}", current_node))?;
        match direction {
            'L' => {
                current_node = &destinations.0;
            }
            'R' => {
                current_node = &destinations.1;
            }
            _ => {
                return Err(format!("Invalid direction {}", direction));
            }
        }

        if current_node == "ZZZ" {
            break;
        }
    }

    Ok(steps)
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    let mut lines = contents.lines();

    let directions = lines.next().ok_or("Missing directions")?.chars().cycle();

    let network = build_network(lines.skip(1))?;

    let mut current_nodes = network
        .keys()
        .filter(|node| node.ends_with('A'))
        .collect::<Vec<_>>();

    let mut steps_to_z = vec![];

    let mut steps = 0u64;
    for direction in directions {
        steps += 1;
        current_nodes = current_nodes
            .into_iter()
            .map(|node| {
                let destinations = network.get(node).ok_or(format!("Missing node {}", node))?;
                match direction {
                    'L' => Ok(&destinations.0),
                    'R' => Ok(&destinations.1),
                    _ => Err(format!("Invalid direction {}", direction)),
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        let num_nodes = current_nodes.len();
        current_nodes.retain(|node| !node.ends_with('Z'));

        // Push `steps` for each filtered-out node that ends with Z
        for _ in 0..num_nodes - current_nodes.len() {
            steps_to_z.push(steps);
        }

        if current_nodes.is_empty() {
            break;
        }
    }

    steps_to_z.into_iter().reduce(num::integer::lcm).ok_or(String::from("No LCM"))
}

fn build_network<'a>(
    lines: impl Iterator<Item = &'a str>,
) -> Result<HashMap<&'a str, (&'a str, &'a str)>, String> {
    let mut network: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in lines {
        let mut line = line.split('=');
        let from = line.next().ok_or("Missing from node")?.trim();
        let mut destinations = line
            .next()
            .ok_or("Missing destination nodes")?
            .trim()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split(',');
        network.insert(
            from,
            (
                destinations
                    .next()
                    .ok_or("Missing left destination")?
                    .trim(),
                destinations
                    .next()
                    .ok_or("Missing right destination")?
                    .trim(),
            ),
        );
    }

    Ok(network)
}
