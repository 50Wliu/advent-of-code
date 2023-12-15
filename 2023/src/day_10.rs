const START: char = 'S';
const NORTH_SOUTH: char = '|';
const EAST_WEST: char = '-';
const NORTH_EAST: char = 'L';
const NORTH_WEST: char = 'J';
const SOUTH_WEST: char = '7';
const SOUTH_EAST: char = 'F';

pub fn part_1(contents: &str) -> Result<u64, String> {
    let lines = contents.lines();
    let (row, starting_line) = lines
        .clone()
        .enumerate()
        .find(|(_, line)| line.contains(START))
        .ok_or("No S")?;
    let col = starting_line.find(START).ok_or("No S")?;

    let grid = lines.collect::<Vec<_>>();

    let steps = traverse(&grid, START, Direction::None, row, col).ok_or("No path found".to_string())?;

    // Farthest distance will be at the halfway point.
    Ok(steps / 2)
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    let lines = contents.lines();

    Ok(0)
}

fn traverse(grid: &[&str], char: char, from: Direction, row: usize, col: usize) -> Option<u64> {
    // If the current pipe can connect to the north,
    // see if the pipe above can also connect to the south.
    if from != Direction::North
        && (char == START || char == NORTH_SOUTH || char == NORTH_WEST || char == NORTH_EAST)
    {
        if let Some(line) = grid.get(row - 1) {
            if let Some(char) = line.chars().nth(col) {
                if char == START {
                    return Some(0);
                }

                if char == NORTH_SOUTH || char == SOUTH_WEST || char == SOUTH_EAST {
                    if let Some(steps) = traverse(grid, char, Direction::South, row - 1, col) {
                        return Some(steps + 1);
                    }
                }
            }
        }
    }

    // If the current pipe can connect to the south,
    // see if the pipe below can also connect to the north.
    if from != Direction::South
        && (char == START || char == NORTH_SOUTH || char == SOUTH_WEST || char == SOUTH_EAST)
    {
        if let Some(line) = grid.get(row + 1) {
            if let Some(char) = line.chars().nth(col) {
                if char == START {
                    return Some(0);
                }

                if char == NORTH_SOUTH || char == NORTH_WEST || char == NORTH_EAST {
                    if let Some(steps) = traverse(grid, char, Direction::North, row + 1, col) {
                        return Some(steps + 1);
                    }
                }
            }
        }
    }

    // If the current pipe can connect to the west,
    // see if the pipe to the left can also connect to the east.
    if from != Direction::West
        && (char == START || char == EAST_WEST || char == NORTH_WEST || char == SOUTH_WEST)
    {
        if let Some(char) = grid[row].chars().nth(col - 1) {
            if char == START {
                return Some(0);
            }

            if char == EAST_WEST || char == NORTH_EAST || char == SOUTH_EAST {
                if let Some(steps) = traverse(grid, char, Direction::East, row, col - 1) {
                    return Some(steps + 1);
                }
            }
        }
    }

    // If the current pipe can connect to the east,
    // see if the pipe to the right can also connect to the west.
    if from != Direction::East
        && (char == START || char == EAST_WEST || char == NORTH_EAST || char == SOUTH_EAST)
    {
        if let Some(char) = grid[row].chars().nth(col + 1) {
            if char == START {
                return Some(0);
            }

            if char == EAST_WEST || char == NORTH_WEST || char == SOUTH_WEST {
                if let Some(steps) = traverse(grid, char, Direction::West, row, col + 1) {
                    return Some(steps + 1);
                }
            }
        }
    }

    None

    // panic!("No connector found at {}, {}", row, col);
}

#[derive(Debug, PartialEq)]
enum Direction {
    None,
    North,
    South,
    East,
    West,
}
