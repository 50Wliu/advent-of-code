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

    update_grid_with_loop(&mut grid, Point { row, col });

    // Farthest distance will be at the halfway point.
    let steps = get_count(&grid, PipeClassification::Loop);
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

    update_grid_with_loop(&mut grid, Point { row, col });

    let mut enclosed_tiles = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            enclosed_tiles += classify_point(&mut grid, Point { row, col });
        }
    }

    for line in &grid {
        for tile in line {
            print!(
                "{}",
                match tile.classification {
                    PipeClassification::Unknown => '?',
                    PipeClassification::Inside => 'I',
                    PipeClassification::Outside => 'O',
                    PipeClassification::Loop => tile.pipe,
                }
            );
        }
        println!();
    }

    // TODO: Why is this not the same as enclosed_tiles?
    // Ok(get_count(&grid, PipeClassification::Inside))
    Ok(enclosed_tiles)
}

fn update_grid_with_loop(grid: &mut [Vec<Tile>], point: Point) {
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
fn get_count(grid: &[Vec<Tile>], classification: PipeClassification) -> u64 {
    let mut count = 0;
    for row in grid {
        for col in row {
            if col.classification == classification {
                count += 1;
            }
        }
    }
    count
}

fn classify_point(grid: &mut [Vec<Tile>], point: Point) -> u64 {
    let Point { row, col } = point;
    let pipe = grid[row][col];
    if pipe.classification != PipeClassification::Unknown {
        return 0;
    }

    let mut already_searched = HashSet::new();
    let mut points_to_search = VecDeque::from([point]);

    while let Some(Point { row, col }) = points_to_search.pop_front() {
        // If we already added this point, don't search it again...
        if !already_searched.insert((row, col)) {
            continue;
        }

        if row == 0 || col == 0 || row == grid.len() - 1 || col == grid[row].len() - 1 {
            let classification = grid[row][col].classification;
            if classification == PipeClassification::Loop {
                // Can't squeeze past the edge of the grid.
                continue;
            } else if classification == PipeClassification::Unknown {
                for (r, c) in &already_searched {
                    grid[*r][*c].classification = PipeClassification::Outside;
                }
                return 0;
            }
        }

        for row_modifier in [-1, 0, 1] {
            for col_modifier in [-1, 0, 1] {
                let point = Point {
                    row: row
                        .checked_add_signed(row_modifier)
                        .expect("Row underflowed even though we checked for row == 0?"),
                    col: col
                        .checked_add_signed(col_modifier)
                        .expect("Col underflowed even though we checked for col == 0?"),
                };
                let Tile {
                    pipe: _,
                    classification,
                } = grid[point.row][point.col];
                match classification {
                    PipeClassification::Unknown => {
                        points_to_search.push_back(point);
                    }
                    PipeClassification::Loop => {
                        if row_modifier == 1 && col_modifier != 1 {
                            points_to_search.append(&mut VecDeque::from(squeeze_through_pipe(
                                grid,
                                Direction::North,
                                &point,
                            )));
                        }

                        if row_modifier != 1 && col_modifier == 1 {
                            points_to_search.append(&mut VecDeque::from(squeeze_through_pipe(
                                grid,
                                Direction::West,
                                &point,
                            )));
                        }

                        if row_modifier != 1 && col_modifier == -1 {
                            points_to_search.append(&mut VecDeque::from(squeeze_through_pipe(
                                grid,
                                Direction::East,
                                &point,
                            )));
                        }

                        if row_modifier == -1 && col_modifier != 1 {
                            points_to_search.append(&mut VecDeque::from(squeeze_through_pipe(
                                grid,
                                Direction::South,
                                &point,
                            )));
                        }
                    }
                    classification => {
                        let mut newly_inside_count = 0;
                        for (r, c) in &already_searched {
                            if grid[*r][*c].classification == PipeClassification::Unknown {
                                newly_inside_count += 1;
                                grid[*r][*c].classification = classification;
                            }
                        }

                        if classification == PipeClassification::Inside {
                            return newly_inside_count;
                        } else {
                            return 0;
                        }
                    }
                }
            }
        }
    }

    let mut newly_inside_count = 0;
    for (r, c) in &already_searched {
        if grid[*r][*c].classification == PipeClassification::Unknown {
            newly_inside_count += 1;
            grid[*r][*c].classification = PipeClassification::Inside;
        }
    }
    newly_inside_count
}

fn squeeze_through_pipe(grid: &[Vec<Tile>], from: Direction, point: &Point) -> Vec<Point> {
    let Point { mut row, mut col } = *point;

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

    // Why we need to search upwards.
    //|   F----7
    //|   |   O|    F---7
    //|   L---7|    |F-7|
    //|   F---J|  F-J|
    //S   L---7L--JOFJ
    //|       |F----J
    //|       ||
    //L-------JL
    //        OO

    loop {
        let tile = grid[row][col];
        if tile.classification != PipeClassification::Loop {
            return vec![Point { row, col }];
        }

        match from {
            Direction::North => {
                let Tile {
                    pipe: adjacent_pipe,
                    classification: adjacent_classification,
                } = grid[row][col + 1];
                if adjacent_classification != PipeClassification::Loop {
                    return vec![Point { row, col }];
                }

                match tile.pipe {
                    SOUTH_WEST | NORTH_SOUTH => {
                        match adjacent_pipe {
                            // 7|, 7F, ||, |F
                            // ??, ??, ??, ??
                            NORTH_SOUTH | SOUTH_EAST => {
                                row += 1;
                            }
                            // 7L, |L
                            // ??, ??
                            NORTH_EAST => {
                                let mut points = squeeze_through_pipe(
                                    grid,
                                    Direction::North,
                                    &Point { row: row + 1, col },
                                );
                                points.append(&mut squeeze_through_pipe(
                                    grid,
                                    Direction::West,
                                    &Point { row, col: col + 1 },
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
                            // J|, JF
                            // ??, ??
                            NORTH_SOUTH | SOUTH_EAST => {
                                let mut points = squeeze_through_pipe(
                                    grid,
                                    Direction::North,
                                    &Point { row: row + 1, col },
                                );
                                points.append(&mut squeeze_through_pipe(
                                    grid,
                                    Direction::East,
                                    &Point { row, col },
                                ));

                                return points;
                            }
                            // JL
                            // ??
                            NORTH_EAST => {
                                let mut points = squeeze_through_pipe(
                                    grid,
                                    Direction::North,
                                    &Point { row: row + 1, col },
                                );
                                points.append(&mut squeeze_through_pipe(
                                    grid,
                                    Direction::West,
                                    &Point { row, col: col + 1 },
                                ));
                                points.append(&mut squeeze_through_pipe(
                                    grid,
                                    Direction::East,
                                    &Point { row, col },
                                ));

                                return points;
                            }
                            _ => {
                                return vec![];
                            }
                        }
                    }
                    _ => {
                        return vec![];
                    }
                }
            }
            Direction::West => {
                let Tile {
                    pipe: adjacent_pipe,
                    classification: adjacent_classification,
                } = grid[row + 1][col];
                if adjacent_classification != PipeClassification::Loop {
                    return vec![Point { row, col }];
                }

                match tile.pipe {
                    EAST_WEST | NORTH_EAST => {
                        match adjacent_pipe {
                            // -?, -?, L?, L?
                            // -?, F?, -?, F?
                            EAST_WEST | SOUTH_EAST => {
                                col += 1;
                            }
                            // -?, L?
                            // 7?, 7?
                            SOUTH_WEST => {
                                let mut points = squeeze_through_pipe(
                                    grid,
                                    Direction::North,
                                    &Point { row: row + 1, col },
                                );
                                points.append(&mut squeeze_through_pipe(
                                    grid,
                                    Direction::West,
                                    &Point { row, col: col + 1 },
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
                            // J?, J?
                            // -?, F?
                            EAST_WEST | SOUTH_EAST => {
                                let mut points = squeeze_through_pipe(
                                    grid,
                                    Direction::West,
                                    &Point { row, col: col + 1 },
                                );
                                points.append(&mut squeeze_through_pipe(
                                    grid,
                                    Direction::South,
                                    &Point { row, col },
                                ));

                                return points;
                            }
                            // J?
                            // 7?
                            SOUTH_WEST => {
                                let mut points = squeeze_through_pipe(
                                    grid,
                                    Direction::North,
                                    &Point { row: row + 1, col },
                                );
                                points.append(&mut squeeze_through_pipe(
                                    grid,
                                    Direction::West,
                                    &Point { row, col: col + 1 },
                                ));
                                points.append(&mut squeeze_through_pipe(
                                    grid,
                                    Direction::South,
                                    &Point { row, col },
                                ));

                                return points;
                            }
                            _ => {
                                return vec![];
                            }
                        }
                    }
                    _ => {
                        return vec![];
                    }
                }
            }
            Direction::East => {
                let Tile {
                    pipe: adjacent_pipe,
                    classification: adjacent_classification,
                } = grid[row + 1][col];
                if adjacent_classification != PipeClassification::Loop {
                    return vec![Point { row, col }];
                }

                match tile.pipe {
                    NORTH_WEST | EAST_WEST => {
                        match adjacent_pipe {
                            // ?J, ?J, ?-, ?-
                            // ?-, ?7, ?-, ?7
                            EAST_WEST | SOUTH_WEST => {
                                col -= 1;
                            }
                            // ?J, ?-
                            // ?F, ?F
                            SOUTH_EAST => {
                                let mut points = squeeze_through_pipe(
                                    grid,
                                    Direction::North,
                                    &Point {
                                        row: row + 1,
                                        col: col - 1,
                                    },
                                );
                                points.append(&mut squeeze_through_pipe(
                                    grid,
                                    Direction::East,
                                    &Point { row, col: col - 1 },
                                ));

                                return points;
                            }
                            _ => {
                                return vec![];
                            }
                        }
                    }
                    NORTH_EAST => {
                        match adjacent_pipe {
                            // ?L, ?L
                            // ?-, ?7
                            EAST_WEST | SOUTH_WEST => {
                                let mut points = squeeze_through_pipe(
                                    grid,
                                    Direction::East,
                                    &Point { row, col: col - 1 },
                                );
                                points.append(&mut squeeze_through_pipe(
                                    grid,
                                    Direction::South,
                                    &Point { row, col: col - 1 },
                                ));

                                return points;
                            }
                            // ?L
                            // ?F
                            SOUTH_EAST => {
                                let mut points = squeeze_through_pipe(
                                    grid,
                                    Direction::North,
                                    &Point {
                                        row: row + 1,
                                        col: col - 1,
                                    },
                                );
                                points.append(&mut squeeze_through_pipe(
                                    grid,
                                    Direction::East,
                                    &Point { row, col: col - 1 },
                                ));
                                points.append(&mut squeeze_through_pipe(
                                    grid,
                                    Direction::South,
                                    &Point { row, col: col - 1 },
                                ));

                                return points;
                            }
                            _ => {
                                return vec![];
                            }
                        }
                    }
                    _ => {
                        return vec![];
                    }
                }
            }
            Direction::South => {
                let Tile {
                    pipe: adjacent_pipe,
                    classification: adjacent_classification,
                } = grid[row][col + 1];
                if adjacent_classification != PipeClassification::Loop {
                    return vec![Point { row, col }];
                }

                match tile.pipe {
                    NORTH_WEST | NORTH_SOUTH => {
                        match adjacent_pipe {
                            // ??, ??, ??, ??
                            // J|, JL, ||, |L
                            NORTH_SOUTH | NORTH_EAST => {
                                row -= 1;
                            }
                            // ??, ??
                            // JF, |F
                            SOUTH_EAST => {
                                let mut points = squeeze_through_pipe(
                                    grid,
                                    Direction::South,
                                    &Point { row: row - 1, col },
                                );
                                points.append(&mut squeeze_through_pipe(
                                    grid,
                                    Direction::West,
                                    &Point {
                                        row: row - 1,
                                        col: col + 1,
                                    },
                                ));

                                return points;
                            }
                            _ => {
                                return vec![];
                            }
                        }
                    }
                    SOUTH_WEST => {
                        match adjacent_pipe {
                            // ??, ??, ??, ??
                            // 7|, 7L, 7|, 7L
                            NORTH_SOUTH | NORTH_EAST => {
                                let mut points = squeeze_through_pipe(
                                    grid,
                                    Direction::South,
                                    &Point { row: row - 1, col },
                                );
                                points.append(&mut squeeze_through_pipe(
                                    grid,
                                    Direction::East,
                                    &Point { row: row - 1, col },
                                ));

                                return points;
                            }
                            // ??
                            // 7F
                            SOUTH_EAST => {
                                let mut points = squeeze_through_pipe(
                                    grid,
                                    Direction::South,
                                    &Point { row: row - 1, col },
                                );
                                points.append(&mut squeeze_through_pipe(
                                    grid,
                                    Direction::West,
                                    &Point {
                                        row: row - 1,
                                        col: col + 1,
                                    },
                                ));
                                points.append(&mut squeeze_through_pipe(
                                    grid,
                                    Direction::East,
                                    &Point { row: row - 1, col },
                                ));

                                return points;
                            }
                            _ => {
                                return vec![];
                            }
                        }
                    }
                    _ => {
                        return vec![];
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
