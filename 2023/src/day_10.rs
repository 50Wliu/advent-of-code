use std::collections::{HashSet, VecDeque};

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

    let mut grid = lines
        .map(|line| {
            line.chars()
                .map(|pipe| Tile {
                    pipe,
                    classification: PipeClassification::Unknown,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    start_traverse_loop(&mut grid, Point { row, col });
    let steps = get_count(&grid);

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

    let mut grid = lines
        .map(|line| {
            line.chars()
                .map(|pipe| Tile {
                    pipe,
                    classification: PipeClassification::Unknown,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    start_traverse_loop(&mut grid, Point { row, col });

    let mut enclosed_tiles = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            enclosed_tiles += classify_point(&mut grid, &Point { row, col });
        }
    }

    Ok(enclosed_tiles)
}

fn start_traverse_loop(grid: &mut [Vec<Tile>], point: Point) {
    let Point { row, col } = point;

    let pipe = grid.to_owned()[row - 1][col].pipe;
    if (pipe == NORTH_SOUTH || pipe == SOUTH_WEST || pipe == SOUTH_EAST)
        && traverse_loop(grid, pipe, Direction::South, Point { row: row - 1, col })
    {
        return;
    }

    let pipe = grid.to_owned()[row + 1][col].pipe;
    if (pipe == NORTH_SOUTH || pipe == NORTH_WEST || pipe == NORTH_EAST)
        && traverse_loop(grid, pipe, Direction::North, Point { row: row + 1, col })
    {
        return;
    }

    let pipe = grid.to_owned()[row][col - 1].pipe;
    if (pipe == EAST_WEST || pipe == NORTH_EAST || pipe == SOUTH_EAST)
        && traverse_loop(grid, pipe, Direction::East, Point { row, col: col - 1 })
    {
        return;
    }

    let pipe = grid.to_owned()[row][col + 1].pipe;
    if (pipe == EAST_WEST || pipe == NORTH_WEST || pipe == SOUTH_WEST)
        && traverse_loop(grid, pipe, Direction::West, Point { row, col: col + 1 })
    {
        return;
    }

    panic!("No path found");
}

fn traverse_loop(
    grid: &mut [Vec<Tile>],
    mut current_pipe: char,
    mut from: Direction,
    point: Point,
) -> bool {
    let Point { mut row, mut col } = point;

    loop {
        grid[row][col] = Tile {
            pipe: current_pipe,
            classification: PipeClassification::Loop,
        };

        if current_pipe == START {
            return true;
        }

        if from != Direction::North
            && (current_pipe == NORTH_SOUTH
                || current_pipe == NORTH_WEST
                || current_pipe == NORTH_EAST)
        {
            // If the current pipe can connect to the north,
            // see if the pipe above can also connect to the south.
            let new_pipe = grid[row - 1][col].pipe;
            if new_pipe == START
                || new_pipe == NORTH_SOUTH
                || new_pipe == SOUTH_WEST
                || new_pipe == SOUTH_EAST
            {
                from = Direction::South;
                current_pipe = new_pipe;
                row -= 1;
            } else {
                return false;
            }
        } else if from != Direction::South
            && (current_pipe == NORTH_SOUTH
                || current_pipe == SOUTH_WEST
                || current_pipe == SOUTH_EAST)
        {
            // If the current pipe can connect to the south,
            // see if the pipe below can also connect to the north.
            let new_pipe = grid[row + 1][col].pipe;
            if new_pipe == START
                || new_pipe == NORTH_SOUTH
                || new_pipe == NORTH_WEST
                || new_pipe == NORTH_EAST
            {
                from = Direction::North;
                current_pipe = new_pipe;
                row += 1;
            } else {
                return false;
            }
        } else if from != Direction::West
            && (current_pipe == EAST_WEST
                || current_pipe == NORTH_WEST
                || current_pipe == SOUTH_WEST)
        {
            // If the current pipe can connect to the west,
            // see if the pipe to the left can also connect to the east.
            let new_pipe = grid[row][col - 1].pipe;
            if new_pipe == START
                || new_pipe == EAST_WEST
                || new_pipe == NORTH_EAST
                || new_pipe == SOUTH_EAST
            {
                from = Direction::East;
                current_pipe = new_pipe;
                col -= 1;
            } else {
                return false;
            }
        } else if from != Direction::East
            && (current_pipe == EAST_WEST
                || current_pipe == NORTH_EAST
                || current_pipe == SOUTH_EAST)
        {
            // If the current pipe can connect to the east,
            // see if the pipe to the right can also connect to the west.
            let new_pipe = grid[row][col + 1].pipe;
            if new_pipe == START
                || new_pipe == EAST_WEST
                || new_pipe == NORTH_WEST
                || new_pipe == SOUTH_WEST
            {
                from = Direction::West;
                current_pipe = new_pipe;
                col += 1;
            } else {
                return false;
            }
        } else {
            panic!(
                "Pipe `{}` at {}, {} cannot connect to anything coming from direction {:?}",
                current_pipe, row, col, from
            );
        }
    }
}

// TODO: expand this to take some matcher
fn get_count(grid: &Vec<Vec<Tile>>) -> u64 {
    let mut count = 0;
    for row in grid {
        for col in row {
            if col.classification == PipeClassification::Loop {
                count += 1;
            }
        }
    }
    count
}

fn classify_point(
    grid: &mut [Vec<Tile>],
    // from: Direction, // TODO: Change this to heading so I don't need to keep flipping the directions in my head.
    point: &Point, // min_loop_point: &Point,
                   // max_loop_point: &Point,
) -> u64 {
    let Point { row, col } = *point;
    // let Point {
    //     row: min_loop_row,
    //     col: min_loop_col,
    // } = *min_loop_point;
    // let Point {
    //     row: max_loop_row,
    //     col: max_loop_col,
    // } = *max_loop_point;

    // if row <= min_loop_row || row >= max_loop_row || col <= min_loop_col || col >= max_loop_col {
    //     return true;
    // }

    let pipe = grid[row][col];
    if pipe.classification != PipeClassification::Unknown {
        return 0;
    }

    let mut already_searched = HashSet::new();
    let mut points_to_search = VecDeque::new();

    // Fill points_to_search with all adjacent points.
    // Continuously dequeue from PTS.
    // If the point we dequeue is unknown, we push 7 more points.
    // !A POINT WE KNOW ABOUT!

    // let mut directions = vec![from];
    // let mut adjacent = None;

    loop {
        // Jinkies! We've made it back to the beginning without finding any way out.
        // if directions.is_empty() {
        //     return false;
        // }

        already_searched.insert((row, col));

        // TODO: Copy this 8 times.
        let Tile {
            pipe: _,
            classification,
        } = grid[row][col - 1];
        match classification {
            PipeClassification::Unknown => {
                points_to_search.push_back(Point { row, col: col - 1 });
            }
            PipeClassification::Loop => {
                points_to_search.append(&mut VecDeque::from(squeeze_through_pipe(
                    grid,
                    Direction::East,
                    &Point { row, col: col - 1 },
                    &Adjacent::Plus,
                )));
            }
            classification => {
                grid[row][col].classification = classification;
                for (r, c) in &already_searched {
                    grid[*r][*c].classification = classification;
                }

                if classification == PipeClassification::Inside {
                    return already_searched.len() as u64;
                } else {
                    return 0;
                }
            }
        }

        // TODO: If points_to_search is empty, we are Inside.

        // ====== OLD ========

        // match directions[directions.len() - 1] {
        //     Direction::North => {
        //         // Are we on a loop segment?
        //         // TODO: Make sure this lines up with the logic described in the non-loop-segment section.
        //         // . |
        //         if let Some(pipe) = r#loop.get(&(row, col)) {
        //             match *pipe {
        //                 NORTH_SOUTH => {
        //                     // If so, check the adjacent pairs for squeezability.
        //                     // TODO: Is this already_searched logic correct?
        //                     // TODO: Once we start squeezing down a pipe, we must stay on the same pair.
        //                     if adjacent.unwrap_or(&Adjacent::Minus) == &Adjacent::Minus {
        //                         if let Some(minus_adjacent) = r#loop.get(&(row, col - 1)) {
        //                             match *minus_adjacent {
        //                                 NORTH_SOUTH
        //                                     if !already_searched.contains(&(row + 1, col)) =>
        //                                 {
        //                                     directions.push(Direction::North);
        //                                     adjacent = Some(&Adjacent::Minus);
        //                                     row += 1;

        //                                     continue;
        //                                 }
        //                                 NORTH_WEST
        //                                     if !already_searched.contains(&(row, col - 1)) =>
        //                                 {
        //                                     directions.push(Direction::East);
        //                                     adjacent = Some(&Adjacent::Minus);
        //                                     col -= 1;

        //                                     continue;
        //                                 }
        //                                 NORTH_EAST
        //                                     if !already_searched.contains(&(row, col + 1)) =>
        //                                 {
        //                                     directions.push(Direction::West);
        //                                     adjacent = Some(&Adjacent::Minus);
        //                                     col += 1;

        //                                     continue;
        //                                 }
        //                                 _ => {
        //                                     // Fall through.
        //                                 }
        //                             }
        //                         }
        //                     }

        //                     // If the first adjacent failed, check the second.
        //                     if adjacent.unwrap_or(&Adjacent::Plus) == &Adjacent::Plus {
        //                         if let Some(plus_adjacent) = r#loop.get(&(row, col + 1)) {
        //                             match *plus_adjacent {
        //                                 NORTH_SOUTH
        //                                     if !already_searched.contains(&(row + 1, col)) =>
        //                                 {
        //                                     directions.push(Direction::North);
        //                                     adjacent = Some(&Adjacent::Plus);
        //                                     row += 1;

        //                                     continue;
        //                                 }
        //                                 NORTH_WEST
        //                                     if !already_searched.contains(&(row, col - 1)) =>
        //                                 {
        //                                     directions.push(Direction::East);
        //                                     adjacent = Some(&Adjacent::Plus);
        //                                     col -= 1;

        //                                     continue;
        //                                 }
        //                                 NORTH_EAST
        //                                     if !already_searched.contains(&(row, col + 1)) =>
        //                                 {
        //                                     directions.push(Direction::West);
        //                                     adjacent = Some(&Adjacent::Plus);
        //                                     col += 1;

        //                                     continue;
        //                                 }
        //                                 _ => {
        //                                     // Fall through.
        //                                 }
        //                             }
        //                         }
        //                     }

        //                     // TODO: Double-check this. What if we find ground?
        //                     // If both failed, we're at a dead end. Backtrack.
        //                     already_searched.insert((row, col));
        //                     directions.pop();
        //                     row -= 1;
        //                 }
        //                 _ => {
        //                     // TODO: Fill in the rest of the cases.
        //                 }
        //             }
        //         } else {
        //             // TODO: This doesn't check if we're going to end up on a loop segment.
        //             // If we are, we need to ensure that
        //             // 1) the segment is oriented correctly, and
        //             // 2) there is a neighboring segment that allows us to squeeze. (Pairs?!?! Ugh)
        //             //    - THERE MAY BE TWO!!!
        //             if !already_searched.contains(&(row + 1, col)) {
        //                 directions.push(Direction::North);
        //                 row += 1;
        //             } else if !already_searched.contains(&(row, col - 1)) {
        //                 directions.push(Direction::East);
        //                 col -= 1;
        //             } else if !already_searched.contains(&(row, col + 1)) {
        //                 directions.push(Direction::West);
        //                 col += 1;
        //             } else {
        //                 // Dead end. Backtrack.
        //                 already_searched.insert((row, col));
        //                 directions.pop();
        //                 row -= 1;
        //             }
        //         }
        //     }
        //     Direction::South => {
        //         // Are we on a loop segment?
        //         if let Some(pipe) = r#loop.get(&(row, col)) {
        //             match *pipe {
        //                 NORTH_SOUTH => {
        //                     already_searched.insert((row, col));
        //                     directions.push(Direction::South);

        //                     row -= 1;
        //                 }
        //                 SOUTH_WEST => {
        //                     already_searched.insert((row, col));
        //                     directions.push(Direction::East);

        //                     col -= 1;
        //                 }
        //                 SOUTH_EAST => {
        //                     already_searched.insert((row, col));
        //                     directions.push(Direction::West);

        //                     col += 1;
        //                 }
        //                 _ => {
        //                     // Dead end. Backtrack.
        //                     directions.pop();
        //                     row += 1;
        //                 }
        //             }
        //         }
        //     }
        //     Direction::East => {
        //         // Are we on a loop segment?
        //         if let Some(pipe) = r#loop.get(&(row, col)) {
        //             match *pipe {
        //                 EAST_WEST => {
        //                     already_searched.insert((row, col));
        //                     directions.push(Direction::East);

        //                     col -= 1;
        //                 }
        //                 NORTH_EAST => {
        //                     already_searched.insert((row, col));
        //                     directions.push(Direction::South);

        //                     row -= 1;
        //                 }
        //                 SOUTH_EAST => {
        //                     already_searched.insert((row, col));
        //                     directions.push(Direction::North);

        //                     row += 1;
        //                 }
        //                 _ => {
        //                     // Dead end. Backtrack.
        //                     directions.pop();
        //                     col += 1;
        //                 }
        //             }
        //         }
        //     }
        //     Direction::West => {
        //         // Are we on a loop segment?
        //         if let Some(pipe) = r#loop.get(&(row, col)) {
        //             match *pipe {
        //                 EAST_WEST => {
        //                     already_searched.insert((row, col));
        //                     directions.push(Direction::West);

        //                     col += 1;
        //                 }
        //                 NORTH_WEST => {
        //                     already_searched.insert((row, col));
        //                     directions.push(Direction::South);

        //                     row -= 1;
        //                 }
        //                 SOUTH_WEST => {
        //                     already_searched.insert((row, col));
        //                     directions.push(Direction::North);

        //                     row += 1;
        //                 }
        //                 _ => {
        //                     // Dead end. Backtrack.
        //                     directions.pop();
        //                     col -= 1;
        //                 }
        //             }
        //         }
        //     }
        // }
    }
}

fn squeeze_through_pipe(
    grid: &[Vec<Tile>],
    from: Direction,
    point: &Point,
    adjacent: &Adjacent,
) -> Vec<Point> {
    let Point { mut row, mut col } = *point;
    let (adjacent_row, adjacent_col) = match from {
        Direction::North | Direction::South => (
            row,
            if adjacent == &Adjacent::Minus {
                col - 1
            } else {
                col + 1
            },
        ),
        Direction::East | Direction::West => (
            if adjacent == &Adjacent::Minus {
                row - 1
            } else {
                row + 1
            },
            col,
        ),
    };
    let adjacent_pipe = grid[adjacent_row][adjacent_col].pipe;

    // OOOOOOOOOOOOOOOOOOOO
    // OF----7F7F7F7F-7OOOO
    // O|F--7||||||||FJOOOO
    // O||OFJ||||||||L7OOOO
    // FJL7L7LJLJ||LJIL-7OO
    // L--JOL7IIILJS7F-7L7O
    // OOOOF-JIIF7FJ|L7L7L7
    // OOOOL7IF7||L7|IL7L7|
    // OOOOO|FJLJ|FJ|F7|OLJ
    // OOOOFJL-7O||O||||OOO
    // OOOOL---JOLJOLJLJOOO

    //|   F----7
    //|   |   O|
    //|   L---7|
    //|       ||  F-7
    //S  .    |L--JO|
    //|       |F----J
    //|       ||
    //L-------JL
    //        OO

    // Counterpoint to hardcoding + 1.
    // (Unless we also check for squeezes in the diagonal directions??)
    //|      F----7
    //|      |O   |
    //|      |F---J
    //|      ||  F-7
    //S      |L--JO|
    //|      |F----J
    //|      ||
    //L------JL
    //       OO

    //
    //---------J0
    //----------7

    // The "7L from the north" case.
    //|   F----7
    //|   |   O|
    //|   L---7|
    //|   F---J|  F-7
    //S   L---7L--JO|
    //|       |F----J
    //|       ||
    //L-------JL
    //        OO

    // 4-way intersection!
    //|   F----7
    //|   |   O|
    //|   L---7|  F-7
    //|   F---JL--JO|
    //S   L---7F----J
    //|       ||
    //|       ||
    //L-------JL
    //        OO

    // What is the base case exit condition?
    // - When the adjacent is not part of the loop, or
    // - When the tile is not part of the loop, or
    // - Anything else...?

    loop {
        let tile = grid[row][col];
        if tile.classification != PipeClassification::Loop {
            return vec![Point { row, col }];
        }

        match from {
            // We are pretty sure we can ignore going up (Direction::South),
            // as we should have already come _down_ from the other direction
            // if iterating top-to-bottom.
            Direction::North => {
                match tile.pipe {
                    // TODO: All of these currently assume Adjacent::Plus.
                    // TODO: Hmm, there's a ton of duplication here...
                    SOUTH_WEST => {
                        match adjacent_pipe {
                            NORTH_SOUTH | SOUTH_EAST => {
                                row += 1;
                            }
                            NORTH_EAST => {
                                // TODO: This one can go both south or east.
                                let mut points = squeeze_through_pipe(
                                    grid,
                                    Direction::North,
                                    &Point { row: row + 1, col },
                                    &Adjacent::Plus,
                                );
                                points.append(&mut squeeze_through_pipe(
                                    grid,
                                    Direction::West,
                                    &Point { row, col: col + 1 },
                                    &Adjacent::Plus,
                                ));

                                return points;
                            }
                            _ => {
                                return vec![];
                            }
                        }
                    }
                    NORTH_WEST => {
                        match adjacent_pipe {
                            NORTH_SOUTH | SOUTH_EAST => {
                                row += 1;
                            }
                            NORTH_EAST => {
                                // TODO: This one can go south, east, AND west (Adjacent::Minus)!
                                let mut points = squeeze_through_pipe(
                                    grid,
                                    Direction::North,
                                    &Point { row: row + 1, col },
                                    &Adjacent::Plus,
                                );
                                points.append(&mut squeeze_through_pipe(
                                    grid,
                                    Direction::West,
                                    &Point { row, col: col + 1 },
                                    &Adjacent::Plus,
                                ));
                                points.append(&mut squeeze_through_pipe(
                                    grid,
                                    Direction::East,
                                    &Point { row: row + 1, col },
                                    &Adjacent::Plus,
                                ));

                                return points;
                            }
                            _ => {
                                return vec![];
                            }
                        }
                    }
                    NORTH_SOUTH => {
                        match adjacent_pipe {
                            NORTH_SOUTH | SOUTH_EAST => {
                                row += 1;
                            }
                            NORTH_EAST => {
                                // TODO: This one can go both south or east.
                                let mut points = squeeze_through_pipe(
                                    grid,
                                    Direction::North,
                                    &Point { row: row + 1, col },
                                    &Adjacent::Plus,
                                );
                                points.append(&mut squeeze_through_pipe(
                                    grid,
                                    Direction::West,
                                    &Point { row, col: col + 1 },
                                    &Adjacent::Plus,
                                ));

                                return points;
                            }
                            _ => {
                                return vec![];
                            }
                        }
                    }
                    _ => todo!(),
                }
            }
            _ => todo!(),
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

#[derive(Clone, Copy, PartialEq)]
enum PipeClassification {
    Unknown,
    Inside,
    Outside,
    Loop,
}

#[derive(Clone, Copy)]
struct Tile {
    pipe: char,
    classification: PipeClassification,
}
