pub fn compute(arg: &str) -> () {
    let data = std::fs::read_to_string(arg).unwrap();
    let mut elves = parse(&data);
    println!("  Part1: {}", elves.iter().max().unwrap());
    elves.sort();
    elves.reverse();
    println!("  Part2: {}", elves[..3].iter().sum::<usize>());
}

fn parse(data: &str) -> Vec<usize> {
    let mut elves = Vec::new();
    let mut sum = 0;
    for line in data.lines() {
        if line.is_empty() {
            elves.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<usize>().unwrap();
        }
    }
    elves.push(sum);

    return elves;
}
