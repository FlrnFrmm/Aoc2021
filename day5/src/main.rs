use std::cmp::{max, min};
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let input = utility::read_input_five(None)?;
    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
    Ok(())
}

fn part_one(input: &Vec<((i32, i32), (i32, i32))>) -> usize {
    let only_straight_lines = get_straight_lines(&input);
    let mut map = HashMap::new();
    for line in only_straight_lines {
        for (x, y) in calculate_points(*line) {
            let entry = map.entry((x, y)).or_insert(0);
            *entry += 1;
        }
    }
    visualize_map(&map);
    map.values().filter(|v| **v > 1).count()
}

fn get_straight_lines(values: &Vec<((i32, i32), (i32, i32))>) -> Vec<&((i32, i32), (i32, i32))> {
    values
        .iter()
        .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
        .collect()
}

fn part_two(input: &Vec<((i32, i32), (i32, i32))>) -> usize {
    let mut map = HashMap::new();
    for line in input {
        for (x, y) in calculate_points(*line) {
            let entry = map.entry((x, y)).or_insert(0);
            *entry += 1;
        }
    }
    visualize_map(&map);
    map.values().filter(|v| **v > 1).count()
}

fn visualize_map(map: &HashMap<(i32, i32), i32>) {
    let (x_max, y_max) = map.keys().fold((0, 0), |(x_acc, y_acc), (x, y)| {
        (max(x_acc, *x), max(y_acc, *y))
    });
    for y in 0..y_max + 1 {
        for x in 0..x_max + 1 {
            let s = map
                .get(&(x, y))
                .map_or(".".to_string(), |v| (*v).to_string());
            print!("{}", s);
        }
        println!("");
    }
}

fn calculate_points(((x1, y1), (x2, y2)): ((i32, i32), (i32, i32))) -> Vec<(i32, i32)> {
    match (x1 == x2, y1 == y2) {
        (true, true) => vec![(x1, y1)],
        (true, false) => calculate_range(y1, y2).iter().map(|y| (x1, *y)).collect(),
        (false, true) => calculate_range(x1, x2).iter().map(|x| (*x, y1)).collect(),
        (false, false) => calculate_range(x1, x2)
            .into_iter()
            .zip(calculate_range(y1, y2).into_iter())
            .collect(),
    }
}

fn calculate_range(left: i32, right: i32) -> Vec<i32> {
    if right > left {
        (left..right + 1).collect()
    } else {
        (right..left + 1).rev().collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn calculate_points() {
        let points = super::calculate_points(((0, 0), (0, 0)));
        assert_eq!(points, vec![(0, 0)]);

        let points = super::calculate_points(((0, 0), (0, 3)));
        assert_eq!(points, vec![(0, 0), (0, 1), (0, 2), (0, 3)]);

        let points = super::calculate_points(((0, 0), (3, 0)));
        assert_eq!(points, vec![(0, 0), (1, 0), (2, 0), (3, 0)]);

        let points = super::calculate_points(((0, 0), (3, 3)));
        assert_eq!(points, vec![(0, 0), (1, 1), (2, 2), (3, 3)]);

        let points = super::calculate_points(((9, 7), (7, 9)));
        assert_eq!(points, vec![(9, 7), (8, 8), (7, 9)]);
    }

    #[test]
    fn calculate_range() {
        let values: Vec<i32> = super::calculate_range(1, 3);
        assert_eq!(values, vec![1, 2, 3]);

        let values: Vec<i32> = super::calculate_range(3, 1);
        assert_eq!(values, vec![3, 2, 1]);
    }
}
