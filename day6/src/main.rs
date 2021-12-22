mod days;

use days::DaysIterator;
use std::collections::{HashMap, HashSet};
fn main() -> anyhow::Result<()> {
    let input = utility::read_input_six(None)?;
    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
    Ok(())
}

fn part_one(input: &Vec<usize>) -> usize {
    let max_day = 79;
    let mut population = init_population(input, max_day);
    for day in 1..=max_day {
        let population_on_day = **&population.get(&day).unwrap_or(&0);
        if population_on_day > 0 {
            for future_day in DaysIterator::new(day, max_day) {
                let future_day_population = population.entry(future_day).or_insert(0);
                *future_day_population += population_on_day;
            }
        }
    }
    population.values().sum::<usize>() + input.len()
}

fn part_two(input: &Vec<usize>) -> usize {
    let max_day = 255;
    let mut population = init_population(input, max_day);
    for day in 1..=max_day {
        let population_on_day = **&population.get(&day).unwrap_or(&0);
        if population_on_day > 0 {
            for future_day in DaysIterator::new(day, max_day) {
                let future_day_population = population.entry(future_day).or_insert(0);
                *future_day_population += population_on_day;
            }
        }
    }
    population.values().sum::<usize>() + input.len()
}

fn init_population(input: &Vec<usize>, max_day: usize) -> HashMap<usize, usize> {
    let mut population: HashMap<usize, usize> = (1..=max_day).map(|v| (v, 0)).collect();

    for day in input {
        for next_day in DaysIterator::new_without_offset(*day, max_day) {
            let entry = population.entry(next_day).or_insert(0);
            *entry += 1;
        }
        let entry = population.entry(*day).or_insert(0);
        *entry += 1;
    }

    population
}

fn print_population(input: &HashMap<usize, usize>, day: usize) {
    println!("Calculation {}:", day);
    for day in day + 1..=input.len() {
        println!("\t Day {}: {:?}", day, input.get(&day));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn calculate_offspring() {
        assert_eq!(0, 0)
    }
}
