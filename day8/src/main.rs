mod decoder;

fn main() -> anyhow::Result<()> {
    let input = utility::read_input_eight(None)?;
    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input)?);
    Ok(())
}

fn part_one(input: &Vec<(Vec<String>, Vec<String>)>) -> usize {
    input
        .iter()
        .map(|(_, output)| {
            output
                .iter()
                .map(|string| match string.len() {
                    2 => Some(()),
                    3 => Some(()),
                    4 => Some(()),
                    7 => Some(()),
                    _ => None,
                })
                .filter(Option::is_some)
                .count()
        })
        .sum()
}

fn part_two(input: &Vec<(Vec<String>, Vec<String>)>) -> anyhow::Result<usize> {
    input
        .iter()
        .map(|(lut, code)| decoder::Decoder::new(lut).decode(code))
        .collect::<anyhow::Result<Vec<usize>>>()
        .map(|values| values.into_iter().sum())
}
