pub fn part_1() -> u32 {
    let contents = super::utilities::read_input(4);
    contents.lines().fold(0, |accum, line| {
        let split_point = line.find(',').expect("Line should contain two assignments");
        let assignment_1 = &line[..split_point];
        let assignment_2 = &line[split_point + 1..];

        let (assignment_1_start, assignment_1_end) = get_assignment_start_and_end(assignment_1);
        let (assignment_2_start, assignment_2_end) = get_assignment_start_and_end(assignment_2);
        if assignment_1_start >= assignment_2_start && assignment_1_end <= assignment_2_end ||
            assignment_2_start >= assignment_1_start && assignment_2_end <= assignment_1_end {
            accum + 1
        } else {
            accum
        }
    })
}

pub fn part_2() -> u32 {
    let contents = super::utilities::read_input(4);
    contents.lines().fold(0, |accum, line| {
        let split_point = line.find(',').expect("Line should contain two assignments");
        let assignment_1 = &line[..split_point];
        let assignment_2 = &line[split_point + 1..];

        let (assignment_1_start, assignment_1_end) = get_assignment_start_and_end(assignment_1);
        let (assignment_2_start, assignment_2_end) = get_assignment_start_and_end(assignment_2);
        if assignment_2_start <= assignment_1_start && assignment_1_start <= assignment_2_end ||
            assignment_2_start <= assignment_1_end && assignment_1_end <= assignment_2_end ||
            assignment_1_start <= assignment_2_start && assignment_2_start <= assignment_1_end ||
            assignment_1_start <= assignment_2_end && assignment_2_end <= assignment_1_end {
            accum + 1
        } else {
            accum
        }
    })
}

fn get_assignment_start_and_end(assignment: &str) -> (u32, u32) {
    let split_point = assignment.find('-').expect("Assignment should be a range of section IDs");
    let assignment_start = &assignment[..split_point];
    let assignment_end = &assignment[split_point + 1..];
    (
        assignment_start.parse().expect("Starting section should be a number"),
        assignment_end.parse().expect("Ending section should be a number")
    )
}
