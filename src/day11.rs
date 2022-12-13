use std::collections::VecDeque;

pub fn compute(arg: &str) -> () {
    let data = std::fs::read_to_string(arg).unwrap();
    let monkey_in = data.split("\n\n");
    let mut monkeys: Vec::<Monkey> = Vec::<Monkey>::new();
    let mut monkeys2: Vec::<Monkey> = Vec::<Monkey>::new();
    for (id, input) in monkey_in.enumerate() {
        let monkey = Monkey::parse(input, id);
        monkeys.push(monkey);
        let monkey2 = Monkey::parse(input, id);
        monkeys2.push(monkey2);
    }

    for round in 0..20 {
        for id in 0..monkeys.len() {
            while !monkeys[id].items.is_empty() {
                let (idx, value) = monkeys[id].throw_next_to();
                monkeys[idx].items.push_back(value);
            }
        }
    }

    let mut inspected: Vec::<usize> = monkeys.iter().map(|m| m.inspected).collect();
    inspected.sort();
    inspected.reverse();
    println!("  Part1: {}", inspected[0]*inspected[1]);

    let modulus = monkeys2.iter().map(|m| m.divisible).collect::<Vec::<usize>>().iter().product();
    for round in 0..10000 {
        for id in 0..monkeys2.len() {
            while !monkeys2[id].items.is_empty() {
                let (idx, value) = monkeys2[id].throw_next_to_mod(modulus);
                monkeys2[idx].items.push_back(value);
            }
        }
    }

    let mut inspected2: Vec::<usize> = monkeys2.iter().map(|m| m.inspected).collect();
    inspected2.sort();
    inspected2.reverse();
    println!("  Part2: {}", inspected2[0]*inspected2[1]);
}

struct Monkey {
    id: usize,
    items: VecDeque<usize>,
    op: Box<dyn Fn(usize) -> usize>,
    divisible: usize,
    true_case: usize,
    false_case: usize,
    inspected: usize,
}

impl Monkey {
    fn parse(input: &str, id: usize) -> Self {
        let lines: Vec::<&str> = input.lines().map(|line| line.split(": ").last().unwrap()).collect();
        Self {
            id: id,
            items: lines[1].split(", ").map(|n| n.parse().unwrap()).collect(),
            op: {
                let op: Vec<_> = lines[2].rsplit_once("= ").unwrap().1.split(' ').collect();
                match op[2] {
                    "old" => Box::new(|old| old * old),
                    b => match (op[1], b.parse::<usize>().unwrap()) {
                        ("+", n) => Box::new(move |old| old + n),
                        ("*", n) => Box::new(move |old| old * n),
                        _ => unreachable!(),
                    }
                }
            },
            divisible: lines[3].rsplit_once(' ').unwrap().1.parse().unwrap(),
            true_case: lines[4].rsplit_once(' ').unwrap().1.parse().unwrap(),
            false_case: lines[5].rsplit_once(' ').unwrap().1.parse().unwrap(),
            inspected: 0,
        }
    }

    fn throw_next_to(&mut self) -> (usize, usize) {
        self.inspected += 1;
        let item = self.items.pop_front().unwrap();
        let new_val: usize = (self.op)(item)/3;
        if new_val % self.divisible == 0 {
            return (self.true_case, new_val);
        } else {
            return (self.false_case, new_val);
        }
    }

    fn throw_next_to_mod(&mut self, modulus: usize) -> (usize, usize) {
        self.inspected += 1;
        let item = self.items.pop_front().unwrap();
        let new_val: usize = (self.op)(item) % modulus;
        if new_val % self.divisible == 0 {
            return (self.true_case, new_val);
        } else {
            return (self.false_case, new_val);
        }
    }

    fn print_items(&self) -> () {
        println!("Monkey {}: {:?}", self.id, self.items);
    }
}
