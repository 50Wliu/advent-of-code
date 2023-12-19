use std::collections::{HashMap, HashSet};

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
        .ok_or(format!("No {}", START))?;
    let col = starting_line.find(START).ok_or(format!("No {}", START))?;

    let grid = lines.collect::<Vec<_>>();

    let steps = start_traverse_loop(&grid, Point { row, col }).len() as u64;

    // Farthest distance will be at the halfway point.
    Ok(steps / 2)
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    let lines = contents.lines();
    let (row, starting_line) = lines
        .clone()
        .enumerate()
        .find(|(_, line)| line.contains(START))
        .ok_or(format!("No {}", START))?;
    let col = starting_line.find(START).ok_or(format!("No {}", START))?;

    let grid = lines.collect::<Vec<_>>();

    let r#loop = start_traverse_loop(&grid, Point { row, col });

    let min_loop_row = r#loop
        .keys()
        .min_by_key(|(row, _)| row)
        .ok_or("Empty loop")?
        .0;
    let min_loop_col = r#loop
        .keys()
        .min_by_key(|(_, col)| col)
        .ok_or("Empty loop")?
        .1;
    let max_loop_row = r#loop
        .keys()
        .max_by_key(|(row, _)| row)
        .ok_or("Empty loop")?
        .0;
    let max_loop_col = r#loop
        .keys()
        .max_by_key(|(_, col)| col)
        .ok_or("Empty loop")?
        .1;

    let mut enclosed_tiles = 0;
    enclosed_tiles = start_traverse_outside(
        &grid,
        &r#loop,
        Point {
            row: min_loop_row,
            col: min_loop_col,
        },
        Point {
            row: max_loop_row,
            col: max_loop_col,
        },
    );
    // for (row, line) in grid.iter().enumerate() {
    //     for (col, _) in line.chars().enumerate() {
    //         // On the loop itself.
    //         if r#loop.get(&(row, col)).is_some() {
    //             continue;
    //         }

    //         if !start_traverse_outside(
    //             &r#loop,
    //             Point { row, col },
    //             Point {
    //                 row: min_loop_row,
    //                 col: min_loop_col,
    //             },
    //             Point {
    //                 row: max_loop_row,
    //                 col: max_loop_col,
    //             },
    //         ) {
    //             enclosed_tiles += 1;
    //         }
    //     }
    // }

    Ok(enclosed_tiles)
}

fn start_traverse_loop(grid: &[&str], point: Point) -> HashMap<(usize, usize), char> {
    let Point { row, col } = point;

    if let Some(line) = grid.get(row - 1) {
        if let Some(char) = line.chars().nth(col) {
            if char == NORTH_SOUTH || char == SOUTH_WEST || char == SOUTH_EAST {
                if let Some(r#loop) =
                    traverse_loop(grid, char, Direction::South, Point { row: row - 1, col })
                {
                    return r#loop;
                }
            }
        }
    }

    if let Some(line) = grid.get(row + 1) {
        if let Some(char) = line.chars().nth(col) {
            if char == NORTH_SOUTH || char == NORTH_WEST || char == NORTH_EAST {
                if let Some(r#loop) =
                    traverse_loop(grid, char, Direction::North, Point { row: row + 1, col })
                {
                    return r#loop;
                }
            }
        }
    }

    if let Some(char) = grid[row].chars().nth(col - 1) {
        if char == EAST_WEST || char == NORTH_EAST || char == SOUTH_EAST {
            if let Some(r#loop) =
                traverse_loop(grid, char, Direction::East, Point { row, col: col - 1 })
            {
                return r#loop;
            }
        }
    }

    if let Some(char) = grid[row].chars().nth(col + 1) {
        if char == EAST_WEST || char == NORTH_WEST || char == SOUTH_WEST {
            if let Some(r#loop) =
                traverse_loop(grid, char, Direction::West, Point { row, col: col + 1 })
            {
                return r#loop;
            }
        }
    }

    panic!("No path found");
}

fn traverse_loop(
    grid: &[&str],
    mut current_pipe: char,
    mut from: Direction,
    point: Point,
) -> Option<HashMap<(usize, usize), char>> {
    let Point { mut row, mut col } = point;
    let mut r#loop = HashMap::new();
    loop {
        r#loop.insert((row, col), current_pipe);

        if current_pipe == START {
            return Some(r#loop);
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
                    if new_pipe == START
                        || new_pipe == NORTH_SOUTH
                        || new_pipe == SOUTH_WEST
                        || new_pipe == SOUTH_EAST
                    {
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
                    if new_pipe == START
                        || new_pipe == NORTH_SOUTH
                        || new_pipe == NORTH_WEST
                        || new_pipe == NORTH_EAST
                    {
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
                if new_pipe == START
                    || new_pipe == EAST_WEST
                    || new_pipe == NORTH_EAST
                    || new_pipe == SOUTH_EAST
                {
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
                if new_pipe == START
                    || new_pipe == EAST_WEST
                    || new_pipe == NORTH_WEST
                    || new_pipe == SOUTH_WEST
                {
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

fn start_traverse_outside(
    grid: &[&str],
    r#loop: &HashMap<(usize, usize), char>,
    min_loop_point: Point,
    max_loop_point: Point,
) -> u64 {
    // let Point { row, col } = point;
    // Start at (0, 0). This is guaranteed to either be outside the loop or on the loop itself.
    let row = 0;
    let col = 0;
    let Point {
        row: min_loop_row,
        col: min_loop_col,
    } = min_loop_point;
    let Point {
        row: max_loop_row,
        col: max_loop_col,
    } = max_loop_point;

    // Already outside.
    // (Perform this check here to prevent underflows.)
    if row <= min_loop_row || row >= max_loop_row || col <= min_loop_col || col >= max_loop_col {
        return 0;
    }

    let _ = traverse_outside(
        r#loop,
        Direction::South,
        &Point { row: row - 1, col },
        &min_loop_point,
        &max_loop_point,
    ) || traverse_outside(
        r#loop,
        Direction::North,
        &Point { row: row + 1, col },
        &min_loop_point,
        &max_loop_point,
    ) || traverse_outside(
        r#loop,
        Direction::East,
        &Point { row, col: col - 1 },
        &min_loop_point,
        &max_loop_point,
    ) || traverse_outside(
        r#loop,
        Direction::West,
        &Point { row, col: col + 1 },
        &min_loop_point,
        &max_loop_point,
    );

    0
}

fn traverse_outside(
    r#loop: &HashMap<(usize, usize), char>,
    from: Direction, // TODO: Change this to heading so I don't need to keep flipping the directions in my head.
    point: &Point,
    min_loop_point: &Point,
    max_loop_point: &Point,
) -> bool {
    let Point { mut row, mut col } = *point;
    let Point {
        row: min_loop_row,
        col: min_loop_col,
    } = *min_loop_point;
    let Point {
        row: max_loop_row,
        col: max_loop_col,
    } = *max_loop_point;

    if row <= min_loop_row || row >= max_loop_row || col <= min_loop_col || col >= max_loop_col {
        return true;
    }

    let mut already_searched = HashSet::new();
    let mut directions = vec![from];
    let mut adjacent = None;

    loop {
        // Jinkies! We've made it back to the beginning without finding any way out.
        if directions.is_empty() {
            return false;
        }

        match directions[directions.len() - 1] {
            Direction::North => {
                // Are we on a loop segment?
                // TODO: Make sure this lines up with the logic described in the non-loop-segment section.
                if let Some(pipe) = r#loop.get(&(row, col)) {
                    match *pipe {
                        NORTH_SOUTH => {
                            // If so, check the adjacent pairs for squeezability.
                            // TODO: Is this already_searched logic correct?
                            // TODO: Once we start squeezing down a pipe, we must stay on the same pair.
                            if adjacent.unwrap_or(&Adjacent::Minus) == &Adjacent::Minus {
                                if let Some(minus_adjacent) = r#loop.get(&(row, col - 1)) {
                                    match *minus_adjacent {
                                        NORTH_SOUTH
                                            if !already_searched.contains(&(row + 1, col)) =>
                                        {
                                            directions.push(Direction::North);
                                            adjacent = Some(&Adjacent::Minus);
                                            row += 1;

                                            continue;
                                        }
                                        NORTH_WEST
                                            if !already_searched.contains(&(row, col - 1)) =>
                                        {
                                            directions.push(Direction::East);
                                            adjacent = Some(&Adjacent::Minus);
                                            col -= 1;

                                            continue;
                                        }
                                        NORTH_EAST
                                            if !already_searched.contains(&(row, col + 1)) =>
                                        {
                                            directions.push(Direction::West);
                                            adjacent = Some(&Adjacent::Minus);
                                            col += 1;

                                            continue;
                                        }
                                        _ => {
                                            // Fall through.
                                        }
                                    }
                                }
                            }

                            // If the first adjacent failed, check the second.
                            if adjacent.unwrap_or(&Adjacent::Plus) == &Adjacent::Plus {
                                if let Some(plus_adjacent) = r#loop.get(&(row, col + 1)) {
                                    match *plus_adjacent {
                                        NORTH_SOUTH
                                            if !already_searched.contains(&(row + 1, col)) =>
                                        {
                                            directions.push(Direction::North);
                                            adjacent = Some(&Adjacent::Plus);
                                            row += 1;

                                            continue;
                                        }
                                        NORTH_WEST
                                            if !already_searched.contains(&(row, col - 1)) =>
                                        {
                                            directions.push(Direction::East);
                                            adjacent = Some(&Adjacent::Plus);
                                            col -= 1;

                                            continue;
                                        }
                                        NORTH_EAST
                                            if !already_searched.contains(&(row, col + 1)) =>
                                        {
                                            directions.push(Direction::West);
                                            adjacent = Some(&Adjacent::Plus);
                                            col += 1;

                                            continue;
                                        }
                                        _ => {
                                            // Fall through.
                                        }
                                    }
                                }
                            }

                            // TODO: Double-check this. What if we find ground?
                            // If both failed, we're at a dead end. Backtrack.
                            already_searched.insert((row, col));
                            directions.pop();
                            row -= 1;
                        }
                        _ => {
                            // TODO: Fill in the rest of the cases.
                        }
                    }
                } else {
                    // TODO: This doesn't check if we're going to end up on a loop segment.
                    // If we are, we need to ensure that
                    // 1) the segment is oriented correctly, and
                    // 2) there is a neighboring segment that allows us to squeeze. (Pairs?!?! Ugh)
                    //    - THERE MAY BE TWO!!!
                    if !already_searched.contains(&(row + 1, col)) {
                        directions.push(Direction::North);
                        row += 1;
                    } else if !already_searched.contains(&(row, col - 1)) {
                        directions.push(Direction::East);
                        col -= 1;
                    } else if !already_searched.contains(&(row, col + 1)) {
                        directions.push(Direction::West);
                        col += 1;
                    } else {
                        // Dead end. Backtrack.
                        already_searched.insert((row, col));
                        directions.pop();
                        row -= 1;
                    }
                }
            }
            Direction::South => {
                // Are we on a loop segment?
                if let Some(pipe) = r#loop.get(&(row, col)) {
                    match *pipe {
                        NORTH_SOUTH => {
                            already_searched.insert((row, col));
                            directions.push(Direction::South);

                            row -= 1;
                        }
                        SOUTH_WEST => {
                            already_searched.insert((row, col));
                            directions.push(Direction::East);

                            col -= 1;
                        }
                        SOUTH_EAST => {
                            already_searched.insert((row, col));
                            directions.push(Direction::West);

                            col += 1;
                        }
                        _ => {
                            // Dead end. Backtrack.
                            directions.pop();
                            row += 1;
                        }
                    }
                }
            }
            Direction::East => {
                // Are we on a loop segment?
                if let Some(pipe) = r#loop.get(&(row, col)) {
                    match *pipe {
                        EAST_WEST => {
                            already_searched.insert((row, col));
                            directions.push(Direction::East);

                            col -= 1;
                        }
                        NORTH_EAST => {
                            already_searched.insert((row, col));
                            directions.push(Direction::South);

                            row -= 1;
                        }
                        SOUTH_EAST => {
                            already_searched.insert((row, col));
                            directions.push(Direction::North);

                            row += 1;
                        }
                        _ => {
                            // Dead end. Backtrack.
                            directions.pop();
                            col += 1;
                        }
                    }
                }
            }
            Direction::West => {
                // Are we on a loop segment?
                if let Some(pipe) = r#loop.get(&(row, col)) {
                    match *pipe {
                        EAST_WEST => {
                            already_searched.insert((row, col));
                            directions.push(Direction::West);

                            col += 1;
                        }
                        NORTH_WEST => {
                            already_searched.insert((row, col));
                            directions.push(Direction::South);

                            row -= 1;
                        }
                        SOUTH_WEST => {
                            already_searched.insert((row, col));
                            directions.push(Direction::North);

                            row += 1;
                        }
                        _ => {
                            // Dead end. Backtrack.
                            directions.pop();
                            col -= 1;
                        }
                    }
                }
            }
        }
    }
}

struct Point {
    row: usize,
    col: usize,
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(PartialEq)]
enum Adjacent {
    Minus,
    Plus,
}

// Loop can change to Outside but not to Inside.
#[derive(PartialEq)]
enum PipeClassification {
    Inside,
    Outside,
    Loop
}
