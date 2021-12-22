use std::collections::HashMap;

#[derive(Debug)]
pub struct Board {
    pub solved: bool,
    pub sum: i32,
    content: HashMap<usize, HashMap<usize, Option<i32>>>,
}

impl Board {
    pub fn from_raw_values(raw_values: &Vec<i32>) -> anyhow::Result<Self> {
        let content = raw_values
            .chunks(5)
            .map(|v| {
                v.iter()
                    .map(|v| Some(*v))
                    .enumerate()
                    .collect::<HashMap<usize, Option<i32>>>()
            })
            .enumerate()
            .collect::<HashMap<usize, HashMap<usize, Option<i32>>>>();

        let board = Self {
            solved: false,
            sum: 0,
            content,
        };
        Ok(board)
    }

    pub fn check_value(&mut self, value: i32) {
        let mut to_check = vec![];
        for i in 0..5usize {
            if let Some(row) = self.content.get_mut(&i) {
                for j in 0..5usize {
                    if let Some(row_value) = row.get_mut(&j) {
                        if let Some(v) = row_value {
                            if *v == value {
                                *row_value = None;
                                to_check.push((i, j));
                            }
                        }
                    }
                }
            }
        }

        for (i, j) in to_check {
            if self.check_row(i) || self.check_column(j) {
                self.solved = true;
                self.sum = self.content.values().fold(0, |acc, row| {
                    acc + row
                        .values()
                        .fold(0, |acc, cell| acc + cell.map_or(0, |v| v))
                })
            }
        }
    }

    fn check_row(&self, row_index: usize) -> bool {
        self.content
            .get(&row_index)
            .map_or(false, |row| row.values().all(Option::is_none))
    }

    fn check_column(&self, col_index: usize) -> bool {
        self.content
            .values()
            .all(|row| row.get(&col_index).map_or(false, Option::is_none))
    }
}
