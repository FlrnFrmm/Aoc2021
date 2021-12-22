mod submarine;

pub use submarine::SubmarineCommand;

pub fn read_input_one(path: Option<&str>) -> anyhow::Result<Vec<i32>> {
    read_raw_content(path)?
        .lines()
        .map(|v| v.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()
        .map_err(anyhow::Error::from)
}

pub fn read_input_two(path: Option<&str>) -> anyhow::Result<Vec<SubmarineCommand>> {
    read_raw_content(path)?
        .lines()
        .map(|v| {
            let mut line_split = v.split(" ");
            let command = line_split.next().unwrap();
            let value = line_split.next().unwrap().parse::<i32>()?;
            SubmarineCommand::from((command, value))
        })
        .collect::<Result<Vec<SubmarineCommand>, _>>()
}

pub fn read_input_three(path: Option<&str>) -> anyhow::Result<Vec<String>> {
    let input = read_raw_content(path)?
        .lines()
        .map(|s| String::from(s))
        .collect::<Vec<String>>();
    Ok(input)
}

pub fn read_input_four(path: Option<&str>) -> anyhow::Result<(Vec<i32>, Vec<Vec<i32>>)> {
    let input = read_raw_content(path)?
        .split("\n\n")
        .map(|s| String::from(s))
        .collect::<Vec<String>>();

    let mut iter = input.iter();

    let numbers = iter
        .next()
        .unwrap()
        .split(",")
        .map(|v| v.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()
        .map_err(anyhow::Error::from)?;

    let mut boards: Vec<Vec<i32>> = vec![vec![]];
    while let Some(values_as_str) = iter.next() {
        let board = values_as_str
            .split_whitespace()
            .map(|v| v.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()
            .map_err(anyhow::Error::from)?;
        boards.push(board);
    }

    Ok((numbers, boards))
}

pub fn read_input_five(path: Option<&str>) -> anyhow::Result<Vec<((i32, i32), (i32, i32))>> {
    read_raw_content(path)?
        .lines()
        .map(|s| {
            let mut iter = s.split(" -> ");

            let mut first_point_iter = iter.next().unwrap().split(',');
            let mut second_point_iter = iter.next().unwrap().split(',');

            let first_point_x = first_point_iter.next().map(|v| v.parse::<i32>()).unwrap();
            let first_point_y = first_point_iter.next().map(|v| v.parse::<i32>()).unwrap();
            let first_point = (first_point_x?, first_point_y?);

            let second_point_x = second_point_iter.next().map(|v| v.parse::<i32>()).unwrap();
            let second_point_y = second_point_iter.next().map(|v| v.parse::<i32>()).unwrap();
            let second_point = (second_point_x?, second_point_y?);

            Ok((first_point, second_point))
        })
        .collect::<anyhow::Result<Vec<((i32, i32), (i32, i32))>>>()
}

pub fn read_input_six(path: Option<&str>) -> anyhow::Result<Vec<usize>> {
    read_raw_content(path)?
        .split(",")
        .map(|s| s.parse::<usize>().map_err(anyhow::Error::from))
        .collect::<anyhow::Result<Vec<usize>>>()
}

pub fn read_input_seven(path: Option<&str>) -> anyhow::Result<Vec<i32>> {
    read_raw_content(path)?
        .split(",")
        .map(|s| s.parse::<i32>().map_err(anyhow::Error::from))
        .collect::<anyhow::Result<Vec<i32>>>()
}

pub fn read_input_eight(path: Option<&str>) -> anyhow::Result<(Vec<String>, Vec<String>)> {
    let mut iter = read_raw_content(path)?.split(" | ");
    let input = iter.next().unwrap().split(" ").map(String::from).collect();
    let output = iter.next().unwrap().split(" ").map(String::from).collect();
    Ok((input, output))
}

fn read_raw_content(path: Option<&str>) -> anyhow::Result<String> {
    let path = path.unwrap_or("input/input.txt");
    std::fs::read_to_string(path).map_err(anyhow::Error::from)
}
