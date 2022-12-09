use std::collections::HashSet;

#[derive(Debug, Default)]
struct Grid {
    knots: Vec<[i64; 2]>,
    size: usize,
    visited: HashSet<[i64; 2]>,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Grid {
    fn new(size: usize) -> Grid {
        Grid{ knots: vec![[0, 0]; size],
              size: size,
              visited: HashSet::new(),
        }
    }

    fn move_head(&mut self, dir: &Direction) -> () {
        match dir {
            Direction::Up => { self.knots[0][0] += 1; },
            Direction::Down => { self.knots[0][0] -= 1; },
            Direction::Left => { self.knots[0][1] -= 1; },
            Direction::Right => { self.knots[0][1] += 1; },
        }
    }

    fn move_knot(&mut self, idx: usize) {
        let prev = self.knots[idx-1];
        let current = &mut self.knots[idx];
        let horizontal = prev[0] - current[0];
        let vertical = prev[1] - current[1];
        
        match (horizontal, vertical) {
            (2, 0) => { current[0] += 1; },
            (-2, 0) => { current[0] -= 1; },
            (0, 2) => { current[1] += 1; },
            (0, -2) => { current[1] -= 1; },
            (1, 2) => { current[0] += 1; current[1] += 1; },
            (1, -2) => { current[0] += 1; current[1] -= 1; },
            (-1, 2) => { current[0] -= 1; current[1] += 1; },
            (-1, -2) => { current[0] -= 1; current[1] -= 1; },
            (2, 1) => { current[1] += 1; current[0] += 1; },
            (-2, 1) => { current[1] += 1; current[0] -= 1; },
            (2, -1) => { current[1] -= 1; current[0] += 1; },
            (-2, -1) => { current[1] -= 1; current[0] -= 1; },
            (2, 2) => { current[0] += 1; current[1] += 1; },
            (2, -2) => { current[0] += 1; current[1] -= 1; },
            (-2, 2) => { current[0] -= 1; current[1] += 1; },
            (-2, -2) => { current[0] -= 1; current[1] -= 1; },
            (_, _) => (),
        }

        if idx == self.size - 1 {
            self.visited.insert(*current);
        } else {
            self.move_knot(idx+1);
        }
    }

    fn make_move(&mut self, dir: &Direction, steps: &usize) -> () {
        for _ in 0..*steps {
            self.move_head(&dir);
            self.move_knot(1);
        }
    }

    fn print(&self) -> () {
        println!("{:?}", self.knots);
    }
}

pub fn compute(arg: &str) -> () {
    let data = std::fs::read_to_string(arg).unwrap();
    let mut grid1: Grid = Grid::new(2);
    let mut grid2: Grid = Grid::new(10);
    for line in data.lines() {
        let movement = parse_line(line);
        grid1.make_move(&movement.0, &movement.1);
        grid2.make_move(&movement.0, &movement.1);
    }

    println!("  Part1: {}", grid1.visited.len());
    println!("  Part2: {}", grid2.visited.len());
}

fn parse_line(line: &str) -> (Direction, usize) {
    let tokens: Vec<&str> = line.split(" ").collect();
    let steps = tokens[1].parse::<usize>().unwrap();
    match tokens[0] {
        "R" => { return (Direction::Right, steps); }, 
        "L" => { return (Direction::Left, steps); },
        "U" => { return (Direction::Up, steps); },
        "D" => { return (Direction::Down, steps); },
        &_ => panic!(),
    }
}
