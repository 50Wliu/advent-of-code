/*
00 01 02 03
10 11 12 13
20 21 22 23
30 31 32 33
*/

pub fn part_1(contents: &str) -> Result<u64, String> {
    let lines = contents.lines();
    let mut galaxy_points = vec![];

    for (row, line) in lines.enumerate() {
        for (col, char) in line.chars().enumerate() {
            match char {
                '.' => continue,
                '#' => galaxy_points.push([row, col]),
                _ => return Err(format!("Invalid character: {}", char)),
            }
        }
    }

    let num_rows = contents.lines().count();
    let num_cols = contents
        .lines()
        .next()
        .ok_or("No next line")?
        .chars()
        .count();

    let empty_rows = find_open_space(&galaxy_points, 0, num_rows);
    let empty_cols = find_open_space(&galaxy_points, 1, num_cols);

    Ok(find_shortest_paths(&galaxy_points, &empty_rows, &empty_cols, 2) as u64)
}

pub fn part_2(contents: &str) -> Result<u64, String> {
    let lines = contents.lines();
    let mut galaxy_points = vec![];

    for (row, line) in lines.enumerate() {
        for (col, char) in line.chars().enumerate() {
            match char {
                '.' => continue,
                '#' => galaxy_points.push([row, col]),
                _ => return Err(format!("Invalid character: {}", char)),
            }
        }
    }

    let num_rows = contents.lines().count();
    let num_cols = contents
        .lines()
        .next()
        .ok_or("No next line")?
        .chars()
        .count();

    let empty_rows = find_open_space(&galaxy_points, 0, num_rows);
    let empty_cols = find_open_space(&galaxy_points, 1, num_cols);

    Ok(find_shortest_paths(&galaxy_points, &empty_rows, &empty_cols, 1_000_000) as u64)
}

fn find_open_space(
    galaxy_points: &[[usize; 2]],
    index_to_look_at: usize,
    range_max: usize
) -> Vec<usize> {
    let mut open_space = (0..range_max).collect::<Vec<_>>();
    for galaxy in galaxy_points {
        let galaxy_location = galaxy[index_to_look_at];
        if open_space.contains(&galaxy_location) {
            open_space.retain(|&element| element != galaxy_location);
        }
    }

    open_space
}

fn find_shortest_paths(
    galaxy_points: &[[usize; 2]],
    empty_rows: &[usize],
    empty_cols: &[usize],
    expanded_size: usize,
) -> u64 {
    let mut shortest_path_sum = 0;
    for (index, galaxy_1) in galaxy_points.iter().enumerate() {
        for galaxy_2 in galaxy_points.iter().skip(index + 1) {
            let [row_1, col_1] = galaxy_1;
            let [row_2, col_2] = galaxy_2;

            let mut rows = [row_1, row_2];
            rows.sort();

            let mut cols = [col_1, col_2];
            cols.sort();

            let num_expanded_rows = empty_rows.iter().filter(|x| (rows[0]..rows[1]).contains(x)).count();
            let num_expanded_cols = empty_cols.iter().filter(|x| (cols[0]..cols[1]).contains(x)).count();

            let row_diff = rows[1] - rows[0] + num_expanded_rows * (expanded_size - 1);
            let col_diff = cols[1] - cols[0] + num_expanded_cols * (expanded_size - 1);

            shortest_path_sum += row_diff + col_diff;
        }
    }

    shortest_path_sum as u64
}
