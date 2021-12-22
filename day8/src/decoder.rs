use std::collections::HashMap;

pub struct Decoder {
    lut: HashMap<usize, String>,
}

impl Decoder {
    pub fn new(code: &Vec<String>) -> Self {
        let mut lut = HashMap::new();

        for code_fragment in code {
            let matched = match code_fragment.len() {
                2 => Some((1, code_fragment)),
                3 => Some((7, code_fragment)),
                4 => Some((4, code_fragment)),
                7 => Some((8, code_fragment)),
                _ => None,
            };
            if let Some((key, value)) = matched {
                lut.entry(key).or_insert(value.clone());
            }
        }

        let mut others: Vec<(usize, String)> = Vec::new();
        for code_fragment in code {
            let matched = match map_against_unique(&lut, code_fragment) {
                (0, 1, 0, 1) => Some((0, code_fragment)),
                (1, 2, 1, 2) => Some((2, code_fragment)),
                (0, 1, 0, 2) => Some((3, code_fragment)),
                (1, 1, 1, 2) => Some((5, code_fragment)),
                (1, 1, 1, 1) => Some((6, code_fragment)),
                (0, 0, 0, 1) => Some((9, code_fragment)),
                _ => None,
            };
            if let Some((key, value)) = matched {
                others.push((key, value.clone()))
            }
        }

        for (key, value) in others {
            lut.entry(key).or_insert(value);
        }

        Self { lut }
    }

    pub fn decode(&self, code: &Vec<String>) -> anyhow::Result<usize> {
        let mut number_as_string = String::new();

        for digit_as_code in code {
            for (number, number_code) in &self.lut {
                if number_code.len() != digit_as_code.len() {
                    continue;
                }
                if digit_as_code.chars().all(|c| number_code.find(c).is_some()) {
                    number_as_string += &number.to_string();
                }
            }
        }

        Ok(number_as_string.parse::<usize>()?)
    }
}

fn from_first_in_second_missing(first: &String, second: &String) -> usize {
    first.chars().filter(|c| second.find(*c).is_none()).count()
}

fn map_against_unique(
    lut: &HashMap<usize, String>,
    number: &String,
) -> (usize, usize, usize, usize) {
    (
        from_first_in_second_missing(lut.get(&1).unwrap(), number),
        from_first_in_second_missing(lut.get(&4).unwrap(), number),
        from_first_in_second_missing(lut.get(&7).unwrap(), number),
        from_first_in_second_missing(lut.get(&8).unwrap(), number),
    )
}

#[cfg(test)]
mod tests {
    use super::from_first_in_second_missing;

    #[test]
    fn it_works() {
        let zero: String = "abcefg".into();
        let four: String = "bcdf".into();
        assert_eq!(from_first_in_second_missing(&four, &zero), 1);

        let one: String = "cf".into();
        assert_eq!(from_first_in_second_missing(&one, &zero), 0);
        assert_eq!(from_first_in_second_missing(&one, &four), 0);

        let eight: String = "abcdefg".into();
        assert_eq!(from_first_in_second_missing(&zero, &eight), 0);
        assert_eq!(from_first_in_second_missing(&one, &eight), 0);
        assert_eq!(from_first_in_second_missing(&four, &eight), 0);
        assert_eq!(from_first_in_second_missing(&eight, &one), 5);
        assert_eq!(from_first_in_second_missing(&eight, &one), 5);
    }
}
