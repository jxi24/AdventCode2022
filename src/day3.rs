use std::collections::HashSet;

fn priority(digit: u8) -> u32 {
    match digit {
        97..=122 => return (digit - 96).into(),
        65..=90 => return (digit - 38).into(),
        _ => panic!()
    }
}

fn find_duplicate(arg: &Vec<char>) -> Option<u32> {
    let mut seen = HashSet::new();
    for i in 0..arg.len()/2 {
        seen.insert(arg[i]);
    }
    for i in arg.len()/2..arg.len() {
        if seen.contains(&arg[i]) {
            return Some(priority(arg[i] as u8));
        }
    }
    println!("No match");
    None
}

fn get_shared(a: &str, b: &str) -> HashSet<char> {
    let (shorter, longer) = if a.len () > b.len() {
        (b, a)
    } else {
        (a, b)
    };

    let set: HashSet<char> = shorter.chars().collect();

    longer.chars().filter(|c| set.contains(&c)).collect()
}

fn find_badge(elves: &[&str; 3]) -> u32 {
    let set1 = get_shared(elves[0], elves[1]);
    let set2 = get_shared(elves[0], elves[2]);

    let mut intersection = set1.intersection(&set2);
    priority(*intersection.next().unwrap() as u8)
}

pub fn compute(arg: &str) -> () {
    let data = std::fs::read_to_string(arg).unwrap();
    let mut sum = 0;
    let mut sum2 = 0;
    let mut elves: [&str; 3] = ["", "", ""];
    for (i, line) in data.lines().enumerate() {
        let char_vec = line.chars().collect();
        sum += find_duplicate(&char_vec).unwrap();
        elves[i % 3] = line;
        if i % 3 == 2 {
            sum2 += find_badge(&elves);
        }
    }
    println!("Part1: {}", sum);
    println!("Part2: {}", sum2);
}
