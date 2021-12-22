use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    let input = utility::read_input_three(None)?;
    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
    Ok(())
}

fn part_one(values: &Vec<String>) -> usize {
    let mut gamma_rate = String::from("");
    let mut epsilon_rate = String::from("");
    for i in 0..values[0].len() {
        let (zeroes, ones) = values.iter().fold((0, 0), |(zeros, ones), string| {
            match string.chars().nth(i).unwrap() {
                '1' => (zeros, ones + 1),
                '0' => (zeros + 1, ones),
                _ => (zeros, ones),
            }
        });
        if zeroes > ones {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        } else {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        }
    }
    let gamma = usize::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon_rate, 2).unwrap();
    gamma * epsilon
}

fn part_two(values: &Vec<String>) -> usize {
    let oxygen_generator_rating = filter_for(values, FilterForOption::Common);
    let co2_scrubber_rating = filter_for(values, FilterForOption::LessCommon);
    oxygen_generator_rating * co2_scrubber_rating
}

enum FilterForOption {
    Common,
    LessCommon,
}

fn filter_for(values: &Vec<String>, filter_for: FilterForOption) -> usize {
    let mut map: HashSet<usize> = HashSet::from_iter(0..values.len());
    for target_index in 0..values[0].len() {
        map = filter_for_inner(values, target_index, &map, &filter_for);
        if map.len() == 1 {
            break;
        }
    }
    let target_index = map.iter().next().unwrap();
    usize::from_str_radix(&values[*target_index], 2).unwrap()
}

fn filter_for_inner(
    values: &Vec<String>,
    target_index: usize,
    map: &HashSet<usize>,
    filter_for: &FilterForOption,
) -> HashSet<usize> {
    let mut new_map = HashSet::new();

    let (zeroes, ones) = map.iter().fold((0, 0), |(zeros, ones), index| {
        match values[*index].chars().nth(target_index).unwrap() {
            '1' => (zeros, ones + 1),
            '0' => (zeros + 1, ones),
            _ => (zeros, ones),
        }
    });

    let common = if zeroes > ones { '0' } else { '1' };

    for index in map.iter() {
        match filter_for {
            FilterForOption::Common => {
                if values[*index].chars().nth(target_index).unwrap() != common {
                    new_map.insert(*index);
                }
            }
            FilterForOption::LessCommon => {
                if values[*index].chars().nth(target_index).unwrap() == common {
                    new_map.insert(*index);
                }
            }
        }
    }

    new_map
}
