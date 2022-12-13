use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Number(u32),
    List(Vec<Packet>),
}

fn split_at_next_comma(arg: &[u8]) -> (&[u8], &[u8]) {
    let mut depth = 0;
    let mut idx = 0;
    loop {
        if idx >= arg.len() {
            return (arg, &[]);
        }

        if arg[idx] == b'[' {
            depth += 1;
        } else if arg[idx] == b']' {
            depth -= 1;
        }

        if arg[idx] == b',' && depth == 0 {
            return (&arg[0..idx], &arg[idx+1..]);
        }

        idx += 1;
    }
}

impl Packet {
    fn parse(arg: &[u8]) -> Option<Self> {
        if arg.is_empty() {
            None
        } else if arg[0] == b'[' {
            let mut result = Vec::new();

            let mut substr = &arg[1..arg.len() - 1];
            while !substr.is_empty() {
                let (head, tail) = split_at_next_comma(substr);
                result.push(Packet::parse(head).unwrap());
                substr = tail;
            }

            Some(Packet::List(result))
        } else {
            Some(Packet::Number(std::str::from_utf8(arg).unwrap().parse().unwrap()))
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Packet::*;

        let mut idx = 0;
        loop {
            let result = match (self, other) {
                (Number(left), Number(right)) => {
                    return left.partial_cmp(right);
                },
                (Number(left), List(_)) => {
                    return (Packet::List(vec![Packet::Number(*left)])).partial_cmp(other);   
                },
                (List(_), Number(right)) => {
                    return self.partial_cmp(&Packet::List(vec![Packet::Number(*right)]));   
                },
                (List(left), List(right)) => {
                    if idx >= left.len() && idx < right.len() {
                        return Some(Ordering::Less);
                    } else if idx < left.len() && idx >= right.len() {
                        return Some(Ordering::Greater);
                    } else if idx >= left.len() || idx >= right.len() {
                        return Some(Ordering::Equal);
                    }

                    left[idx].partial_cmp(&right[idx])
                }
            };

            match result {
                Some(Ordering::Equal) => idx += 1,
                _ => return result,
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn compute(arg: &str) -> () {
    let data = std::fs::read_to_string(arg).unwrap();
    let pairs = data.split("\n\n");
    let mut ordered = 0;
    for (idx, pair) in pairs.enumerate() {
        let (lhs_in, rhs_in) = pair.trim().split_once("\n").unwrap();
        let lhs = Packet::parse(lhs_in.as_bytes());
        let rhs = Packet::parse(rhs_in.as_bytes());
        if lhs < rhs {
            ordered += idx+1;
        }
    }

    println!("  Part1: {}", ordered);

    let mut packets = data.lines().filter_map(|line| Packet::parse(line.as_bytes())).collect::<Vec<_>>();
    let divider_0 = Packet::parse("[[2]]".as_bytes()).unwrap();
    let divider_1 = Packet::parse("[[6]]".as_bytes()).unwrap();
    packets.push(divider_0.clone());
    packets.push(divider_1.clone());

    packets.sort();

    let idx0 = packets.binary_search(&divider_0).unwrap();
    let idx1 = packets.binary_search(&divider_1).unwrap();

    println!("  Part2: {}", (idx0 + 1)*(idx1 + 1));
}
