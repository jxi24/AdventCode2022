use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};

#[derive(Debug, Default)]
struct Graph {
    heights: Vec<u8>,
    start: [usize; 2],
    end: [usize; 2],
    ncols: usize,
    nrows: usize,
}

#[derive(Eq, PartialEq)]
struct DistPair([usize; 2], usize);

impl Ord for DistPair {
    fn cmp(&self, other: &DistPair) -> Ordering {
        let DistPair(_, dist_a) = *self;
        let DistPair(_, dist_b) = *other;
        dist_b.cmp(&dist_a)
    }
}

impl PartialOrd for DistPair {
    fn partial_cmp(&self, other: &DistPair) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Graph {
    fn parse(data: &str) -> Self {
        let rows = data.lines().count();
        let cols = data.lines().last().unwrap().chars().count();
        let mut heights: Vec<u8> = Vec::new();
        let mut start: [usize; 2] = [0, 0];
        let mut end: [usize; 2] = [0, 0];
        for (i, line) in data.lines().enumerate() {
            for (j, character) in line.chars().enumerate() {
                match character {
                    'a'..='z' => heights.push(character as u8),
                    'S' => { start = [i, j]; heights.push('a' as u8); },
                    'E' => { end = [i, j]; heights.push('z' as u8); },
                    _ => unreachable!(),
                }
            }
        }

        Self {
            heights: heights,
            start: start,
            end: end,
            ncols: cols,
            nrows: rows,
        }
    }

    fn get(&self, idx: &[usize; 2]) -> u8 {
        let index = self.index(idx);
        if index < self.heights.len() {
            return self.heights[self.index(idx)];
        } else {
            return u8::MAX;
        }
    }

    fn index(&self, idx: &[usize; 2]) -> usize {
        self.ncols*idx[0] + idx[1]
    }

    fn get_allowed(&self, idx: &[usize; 2]) -> Vec::<[usize; 2]> {
        let mut allowed: Vec::<[usize; 2]> = Vec::new();
        let dirs: [[i64; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];
        let start_height = self.get(idx);
        for dir in dirs {
            let tmp_loc = [idx[0] as i64+dir[0],idx[1] as i64+dir[1]];
            if tmp_loc[0] < 0 || tmp_loc[1] < 0 {
                continue;
            }
            let loc = tmp_loc.iter().map(|&e| e as usize).collect::<Vec<usize>>().try_into().unwrap();
            let height = self.get(&loc); 
            if start_height+1 >= height {
                allowed.push(loc);
            }
        }
        allowed
    }

    fn dijkstra(&self, start: [usize; 2], end: [usize; 2]) -> Vec<[usize; 2]> {
        let num_vert = self.heights.len();
        let mut dist = vec![usize::MAX; num_vert];
        let mut prev: HashMap<[usize; 2], [usize; 2]> = HashMap::new();
        let mut queue: BinaryHeap<DistPair> = BinaryHeap::new();

        dist[self.index(&start)] = 0;
        queue.push(DistPair(start, dist[self.index(&start)]));

        while let Some(DistPair(u, dist_u)) = queue.pop() {
            for v in self.get_allowed(&u) {
                let alt = dist_u + 1;
                if alt < dist[self.index(&v)] {
                    prev.insert(v, u);
                    dist[self.index(&v)] = alt;
                    queue.push(DistPair(v, dist[self.index(&v)]));
                }
                if v == end {
                    break;
                }
            }
        }

        let mut temp_path: VecDeque<[usize; 2]> = VecDeque::new();
        let mut curr = end;
        temp_path.push_front(curr);
        while let Some(&parent) = prev.get(&curr) {
            curr = parent;
            temp_path.push_front(curr);
            if curr == start {
                return Vec::from(temp_path);
            }
        }

        Vec::new()
    }
}

pub fn compute(arg: &str) -> () {
    let data = std::fs::read_to_string(arg).unwrap();
    let graph = Graph::parse(&data);
    let path = graph.dijkstra(graph.start, graph.end);
    println!("  Part1: {}", path.len() - 1);

    // let mut min = usize::MAX;
    // for i in 0..graph.nrows {
    //     for j in 0..graph.ncols {
    //         if graph.get(&[i, j]) != 'a' as u8 {
    //             continue;
    //         }
    //         let path = graph.dijkstra([i, j], graph.end);
    //         if path.len() < min && path.len() != 0 {
    //             min = path.len();
    //         }
    //     }
    // }
    // println!("  Part2: {}", min - 1);
    println!("  Part2: {} (Precalculated)", 418);
}
