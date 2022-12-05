pub fn compute(arg: &str) -> () {
    let data = std::fs::read_to_string(arg).unwrap();
    let (crates_str, moves) = data.split_once("\n\n").unwrap();
    let size = crates_str.lines().last().unwrap()
                         .split_terminator(" ").last().unwrap()
                         .parse::<usize>().unwrap();
    let mut crates1 = parse_crates(&crates_str, size);
    let part1 = move_crates_part1(moves, &mut crates1);
    println!("  Part1: {part1}");

    let mut crates2 = parse_crates(&crates_str, size);
    let part2 = move_crates_part2(moves, &mut crates2);
    println!("  Part2: {part2}");
}

fn parse_crates(arg: &str, size: usize) -> Vec<Vec<char>> {
    let mut result = vec![Vec::new(); size];

    for i in arg.lines().filter(|x| !x.starts_with(" 1")) {
        for j in 0..size {
            if let Some(x) = i.chars().nth(1 + j * 4) {
                if x.is_whitespace() {
                    continue;
                }

                result[j].push(x);
            }
        }
    }

    result
}

fn move_crates_part1(moves: &str, crates: &mut Vec<Vec<char>>) -> String {
    for i in moves.lines() {
        let tokens = i.split_whitespace().collect::<Vec<_>>();
        let count = tokens[1].parse::<usize>().unwrap();
        let from = tokens[3].parse::<usize>().unwrap() - 1;
        let to = tokens[5].parse::<usize>().unwrap() - 1;

        for _ in 0..count {
            let old = crates[from].remove(0);
            crates[to].insert(0, old);
        }
    }

    let mut result = String::new();
    for i in crates.iter().filter(|x| !x.is_empty()) {
        result.push(i[0]);
    }

    result
}

fn move_crates_part2(moves: &str, crates: &mut Vec<Vec<char>>) -> String {
    for i in moves.lines() {
        let tokens = i.split_whitespace().collect::<Vec<_>>();
        let count = tokens[1].parse::<usize>().unwrap();
        let from = tokens[3].parse::<usize>().unwrap() - 1;
        let to = tokens[5].parse::<usize>().unwrap() - 1;

        let mut stack = Vec::new();
        for _ in 0..count {
            let old = crates[from].remove(0);
            stack.push(old);
        }

        for old in stack.iter().rev() {
            crates[to].insert(0, *old);
        }
    }

    let mut result = String::new();
    for i in crates.iter().filter(|x| !x.is_empty()) {
        result.push(i[0]);
    }

    result
}
