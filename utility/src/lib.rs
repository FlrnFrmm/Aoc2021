use std::fs;

pub fn read_input(path: Option<&str>) -> anyhow::Result<Vec<i32>> {
    let path = path.unwrap_or("input/input.txt");
    let input = fs::read_to_string(path)?;
    let values: Vec<i32> = input.lines().map(|v| v.parse::<i32>().unwrap()).collect();
    Ok(values)
}
