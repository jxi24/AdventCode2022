pub fn compute(arg: &str) -> () {
    let data = std::fs::read_to_string(arg).unwrap();
    let pairs = parse(&data);
    let result = calc_result(&pairs);
    println!("  Part1: {}", result.0);
    println!("  Part2: {}", result.1);
}

fn parse(data: &str) -> Vec<Vec<usize>> {
    let mut pairs = Vec::new();
    for line in data.lines() {
        pairs.push(line.split(&[',', '-'][..])
                   .map(|x| x.parse::<usize>().unwrap())
                   .collect());
    }
    pairs
}

fn calc_result(pairs: &Vec<Vec<usize>>) -> (usize, usize) {
    let mut contained = 0;
    let mut overlap = 0;
    for pair in pairs {
        if pair[0] <= pair[2] && pair[1] >= pair[3] {
            contained += 1;
        } else if pair[2] <= pair[0] && pair[3] >= pair[1] {
            contained += 1;
        }
        if !(pair[0] > pair[3] || pair[1] < pair[2]) {
            overlap += 1;
        }
    }
    (contained, overlap)
}
