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

    let steps = start_traverse(&grid, row, col);

    // Farthest distance will be at the halfway point.
    Ok(steps / 2)
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    let lines = contents.lines();

    Ok(0)
}

fn start_traverse(grid: &[&str], row: usize, col: usize) -> u64 {
    if let Some(line) = grid.get(row - 1) {
        if let Some(char) = line.chars().nth(col) {
            if char == NORTH_SOUTH || char == SOUTH_WEST || char == SOUTH_EAST {
                if let Some(steps) = traverse(grid, char, Direction::South, row - 1, col) {
                    return steps;
                }
            }
        }
    }

    if let Some(line) = grid.get(row + 1) {
        if let Some(char) = line.chars().nth(col) {
            if char == NORTH_SOUTH || char == NORTH_WEST || char == NORTH_EAST {
                if let Some(steps) = traverse(grid, char, Direction::North, row + 1, col) {
                    return steps;
                }
            }
        }
    }

    if let Some(char) = grid[row].chars().nth(col - 1) {
        if char == EAST_WEST || char == NORTH_EAST || char == SOUTH_EAST {
            if let Some(steps) = traverse(grid, char, Direction::East, row, col - 1) {
                return steps;
            }
        }
    }

    if let Some(char) = grid[row].chars().nth(col + 1) {
        if char == EAST_WEST || char == NORTH_WEST || char == SOUTH_WEST {
            if let Some(steps) = traverse(grid, char, Direction::West, row, col + 1) {
                return steps;
            }
        }
    }

    panic!("No path found");
}

fn traverse(
    grid: &[&str],
    mut current_pipe: char,
    mut from: Direction,
    mut row: usize,
    mut col: usize,
) -> Option<u64> {
    let mut steps = 0;
    loop {
        steps += 1;

        if current_pipe == START {
            return Some(steps);
        }

        if from != Direction::North
            && (current_pipe == NORTH_SOUTH
                || current_pipe == NORTH_WEST
                || current_pipe == NORTH_EAST)
        {
            // If the current pipe can connect to the north,
            // see if the pipe above can also connect to the south.
            if let Some(line) = grid.get(row - 1) {
                if let Some(new_pipe) = line.chars().nth(col) {
                    if new_pipe == START || new_pipe == NORTH_SOUTH || new_pipe == SOUTH_WEST || new_pipe == SOUTH_EAST {
                        from = Direction::South;
                        current_pipe = new_pipe;
                        row -= 1;
                    } else {
                        return None;
                    }
                }
            }
        } else if from != Direction::South
            && (current_pipe == NORTH_SOUTH
                || current_pipe == SOUTH_WEST
                || current_pipe == SOUTH_EAST)
        {
            // If the current pipe can connect to the south,
            // see if the pipe below can also connect to the north.
            if let Some(line) = grid.get(row + 1) {
                if let Some(new_pipe) = line.chars().nth(col) {
                    if new_pipe == START || new_pipe == NORTH_SOUTH || new_pipe == NORTH_WEST || new_pipe == NORTH_EAST {
                        from = Direction::North;
                        current_pipe = new_pipe;
                        row += 1;
                    } else {
                        return None;
                    }
                }
            }
        } else if from != Direction::West
            && (current_pipe == EAST_WEST
                || current_pipe == NORTH_WEST
                || current_pipe == SOUTH_WEST)
        {
            // If the current pipe can connect to the west,
            // see if the pipe to the left can also connect to the east.
            if let Some(new_pipe) = grid[row].chars().nth(col - 1) {
                if new_pipe == START || new_pipe == EAST_WEST || new_pipe == NORTH_EAST || new_pipe == SOUTH_EAST {
                    from = Direction::East;
                    current_pipe = new_pipe;
                    col -= 1;
                } else {
                    return None;
                }
            }
        } else if from != Direction::East
            && (current_pipe == EAST_WEST
                || current_pipe == NORTH_EAST
                || current_pipe == SOUTH_EAST)
        {
            // If the current pipe can connect to the east,
            // see if the pipe to the right can also connect to the west.
            if let Some(new_pipe) = grid[row].chars().nth(col + 1) {
                if new_pipe == START || new_pipe == EAST_WEST || new_pipe == NORTH_WEST || new_pipe == SOUTH_WEST {
                    from = Direction::West;
                    current_pipe = new_pipe;
                    col += 1;
                } else {
                    return None;
                }
            }
        } else {
            panic!(
                "Pipe `{}` at {}, {} cannot connect to anything coming from direction {:?}",
                current_pipe, row, col, from
            );
        }
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}
