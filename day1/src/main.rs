fn main() -> anyhow::Result<()> {
    let values = utility::read_input(None)?;

    println!("Part One: {}", part_one(&values));

    println!("Part Two: {}", part_two(&values));

    Ok(())
}

fn part_one(values: &Vec<i32>) -> usize {
    values
        .iter()
        .skip(1)
        .fold((0usize, 0i32), |(mut count, last), v| {
            if *v > last {
                count += 1;
            }
            (count, *v)
        })
        .0
}

fn part_two(values: &Vec<i32>) -> usize {
    let mut last = values[..3].iter().sum::<i32>();
    let mut count = 0;
    for i in 1..values.len() - 2 {
        let next = values[i..i + 3].iter().sum::<i32>();
        if next > last {
            count += 1;
        }
        last = next;
    }
    count
}
