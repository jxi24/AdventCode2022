use std::collections::HashSet;

#[derive(Debug)]
pub struct CircularBuffer<T> {
    start: usize,
    data: Vec<T>,
}

impl<T: std::hash::Hash + std::cmp::Eq> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            start: 0,
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, item: T) {
        if self.data.len() < self.data.capacity() {
            self.data.push(item);
        } else {
            self.data[self.start] = item;
            self.start += 1;
            if self.start == self.data.capacity() {
                self.start = 0;
            }
        }
    }

    pub fn all_unique(&self) -> bool {
        let mut seen = HashSet::new();
        for point in self.data.iter() {
            if seen.contains(&point)  {
                return false;
            }
            seen.insert(point);
        }
        true
    }
}

pub fn compute(arg: &str) -> () {
    let data = std::fs::read_to_string(arg).unwrap();
    let mut buffer = CircularBuffer::new(4);
    let mut buffer2 = CircularBuffer::new(14);
    let mut found = false;
    for (i, c) in data.chars().enumerate() {
        buffer.push(c);
        buffer2.push(c);
        if buffer.all_unique() && i > 3 && !found {
            println!("  Part1: {}", i+1);
            found = true;
        }
        if buffer2.all_unique() && i > 13 {
            println!("  Part2: {}", i+1);
            break;
        }
    }
}
