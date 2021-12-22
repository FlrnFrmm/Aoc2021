fn main() -> anyhow::Result<()> {
    let input = utility::read_input_seven(None)?;
    // let input = utility::read_input_seven(Some("./input/example.txt"))?;
    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
    Ok(())
}

fn part_one(input: &Vec<i32>) -> i32 {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();
    (min..=max)
        .fold((-1, i32::MAX), |(position, fuel), v| {
            let v_fuel = calculate_distance_part_one(input, v);
            println!("Position {}: {}", v, v_fuel);
            if v_fuel < fuel {
                (v, v_fuel)
            } else {
                (position, fuel)
            }
        })
        .1
}

fn part_two(input: &Vec<i32>) -> i32 {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();
    (min..=max)
        .fold((-1, i32::MAX), |(position, fuel), v| {
            let v_fuel = calculate_distance_part_two(input, v);
            println!("Position {}: {}", v, v_fuel);
            if v_fuel < fuel {
                (v, v_fuel)
            } else {
                (position, fuel)
            }
        })
        .1
}

fn calculate_distance_part_one(values: &Vec<i32>, value: i32) -> i32 {
    values.iter().fold(0, |acc, v| acc + (value - v).abs())
}

fn calculate_distance_part_two(values: &Vec<i32>, value: i32) -> i32 {
    values.iter().fold(0, |acc, v| {
        let fuel: i32 = (0..=(value - v).abs()).sum();
        acc + fuel
    })
}
