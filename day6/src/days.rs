pub struct DaysIterator {
    current: usize,
    max: usize,
    offset: bool,
}

impl DaysIterator {
    pub fn new(start: usize, max: usize) -> Self {
        Self {
            current: start,
            max,
            offset: true,
        }
    }

    pub fn new_without_offset(start: usize, max: usize) -> Self {
        Self {
            current: start,
            max,
            offset: false,
        }
    }
}

impl Iterator for DaysIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = 7;

        if self.offset {
            self.offset = false;
            next += 2;
        };

        if (self.current + next) > self.max {
            return None;
        }

        self.current += next;
        Some(self.current)
    }
}
