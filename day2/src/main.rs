use utility::SubmarineCommand;

fn main() -> anyhow::Result<()> {
    let input = utility::read_input_two(None)?;
    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
    Ok(())
}

fn part_one(values: &Vec<SubmarineCommand>) -> i32 {
    let position = values.iter().fold((0, 0), |(x, y), cmd| match cmd {
        SubmarineCommand::Forward(v) => (x + v, y),
        SubmarineCommand::Up(v) => (x, y - v),
        SubmarineCommand::Down(v) => (x, y + v),
    });
    position.0 * position.1
}

fn part_two(values: &Vec<SubmarineCommand>) -> i32 {
    let position = values.iter().fold((0, 0, 0), |(x, y, aim), cmd| match cmd {
        SubmarineCommand::Forward(v) => (x + v, y + (aim * v), aim),
        SubmarineCommand::Up(v) => (x, y, aim - v),
        SubmarineCommand::Down(v) => (x, y, aim + v),
    });
    position.0 * position.1
}
