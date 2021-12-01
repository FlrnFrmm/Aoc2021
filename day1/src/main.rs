fn main() -> anyhow::Result<()> {
    let values = utility::read_input(None)?;
    println!("Part One: {}", part_one(&vec![]));
    println!("Part Two: {}", part_two(&values));
    Ok(())
}

fn part_one(values: &Vec<i32>) -> usize {
    if values.len() <= 0 {
        return 0;
    }
    values
        .iter()
        .skip(1)
        .fold((0usize, values[0]), |(mut count, last), v| {
            if *v > last {
                count += 1;
            }
            (count, *v)
        })
        .0
}

fn part_two(values: &Vec<i32>) -> usize {
    let values = values
        .windows(3)
        .map(|vs| vs.iter().sum::<i32>())
        .collect::<Vec<i32>>();
    part_one(&values)
}
