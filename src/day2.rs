use std::collections::HashMap;

fn cal_winner_part1(p1: &usize, p2: &usize) -> usize {
    let score = match (p1, p2) {
        (1, 1) | (2, 2) | (3, 3) => { 3 }
        (1, 3) | (2, 1) | (3, 2) => { 0 }
        (3, 1) | (1, 2) | (2, 3) => { 6 }
        (&_, _) => panic!()
    };

    return score + p2;
}

fn cal_winner_part2(p1: &usize, p2: &usize) -> usize {
    let score = match (p1, p2) {
        (1, 1) => { 3 }
        (2, 1) => { 1 }
        (3, 1) => { 2 }
        (1, 2) => { 4 }
        (2, 2) => { 5 }
        (3, 2) => { 6 }
        (1, 3) => { 8 }
        (2, 3) => { 9 }
        (3, 3) => { 7 }
        (&_, _) => panic!()
    };

    return score;
}

pub fn compute(arg: &str) -> () {
    let scoring = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
        ("A", 1),
        ("B", 2),
        ("C", 3)]);

    let data = std::fs::read_to_string(arg).unwrap();
    let mut score_part1 = 0;
    let mut score_part2 = 0;
    for line in data.lines() {
        let plays: Vec<&str> = line.split(" ").collect();
        let p1 = scoring[plays[0]];
        let p2 = scoring[plays[1]];
        score_part1 += cal_winner_part1(&p1, &p2);
        score_part2 += cal_winner_part2(&p1, &p2);
    }

    println!("Part1: {}", score_part1);
    println!("Part2: {}", score_part2);
}
