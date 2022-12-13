pub fn compute(arg: &str) -> () {
    let data = std::fs::read_to_string(arg).unwrap();
    let mut comp: Computer = Computer::new();
    let mut crt: Display = Display::new();
    for line in data.lines() {
        match line {
            x if x.contains("addx") => {
                comp.increment_cycle();
                crt.increment(&comp.reg_x);
                comp.increment_cycle();
                crt.increment(&comp.reg_x);
                let value = x.split(" ").collect::<Vec<&str>>()[1].parse::<i64>().unwrap();
                comp.addx(value);
            }
            &_ => { 
                comp.increment_cycle();
                crt.increment(&comp.reg_x);
            }
        }
    }

    println!("  Part1: {}", comp.result);
    println!("  Part2:");
    crt.draw();
}

struct Computer {
    cycle: usize,
    reg_x: i64,
    checks: usize,
    result: i64,
}

impl Computer {
    fn new() -> Self {
        Self {
            cycle: 0,
            reg_x: 1,
            checks: 20,
            result: 0,
        }
    }

    fn increment_cycle(&mut self) -> () {
        self.cycle += 1;
        if self.cycle == self.checks {
            self.result += self.reg_x*(self.checks as i64);
            self.checks += 40;
        }
    }

    fn addx(&mut self, value: i64) -> () {
        self.reg_x += value;
    }
}

struct Display {
    display: Vec<char>,
    rows: usize,
    cols: usize,
    cycle: i64,
}

impl Display {
    fn new() -> Self {
        Self {
            display: vec!['.'; 240],
            rows: 6,
            cols: 40,
            cycle: 0,
        }
    }

    fn increment(&mut self, reg_x: &i64) -> () {
        if (reg_x - (self.cycle % 40)).abs() <= 1 {
            self.display[self.cycle as usize] = '#';    
        }
        self.cycle += 1;
    }

    fn draw(&self) -> () {
        for i in 0..self.rows {
            print!("  ");
            for j in 0..self.cols {
                print!("{}", self.display[i*self.cols+j]);
            }
            print!("\n");
        }
    }
}
