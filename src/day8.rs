#[derive(Debug, Default)]
struct Forest {
    trees: Vec<i32>,
    visible: Vec<bool>,
    scenic: Vec<usize>,
    nrows: usize,
    ncols: usize,
}

enum Step {
    Up,
    Down,
    Left,
    Right,
}

impl Forest {
    fn parse(&mut self, data: &str) -> () {
        self.trees = Vec::new();
        self.visible = Vec::new();
        self.nrows = data.lines().count();
        for (i, line) in data.lines().enumerate() {
            if i == 0 {
                self.ncols = line.chars().count();
            }
            for tree in line.chars() {
                self.trees.push(tree.to_digit(10).unwrap() as i32);
                self.visible.push(false);
            }
        }
    }

    fn find_visible(&mut self) -> () {
        let mut max: [i32; 2];
        for row in 0..self.nrows {
            max = [-1, -1];
            for col in 0..self.ncols {
                let index_f = row*self.ncols + col;
                let index_b = row*self.ncols + self.ncols - col - 1;
                let tree_f = self.trees[index_f]; 
                if tree_f > max[0] {
                    self.visible[index_f] = true;
                    max[0] = tree_f;
                }

                let tree_b = self.trees[index_b]; 
                if tree_b > max[1] {
                    self.visible[index_b] = true;
                    max[1] = tree_b;
                }
            }
        }
        
        for col in 0..self.ncols {
            max = [-1, -1];
            for row in 0..self.nrows {
                let index_f = row*self.ncols + col;
                let index_b = (self.nrows-row-1)*self.ncols + col;
                let tree_f = self.trees[index_f]; 
                if tree_f > max[0] {
                    self.visible[index_f] = true;
                    max[0] = tree_f;
                }

                let tree_b = self.trees[index_b]; 
                if tree_b > max[1] {
                    self.visible[index_b] = true;
                    max[1] = tree_b;
                }
            }
        }
    }

    fn count_visible(&self) -> usize {
        self.visible.iter().filter(|x| **x).count()
    }

    fn print_visible(&self) -> () {
        for row in 0..self.nrows {
            for col in 0..self.ncols {
                let index = row*self.ncols+col;
                let visible = if self.visible[index] {"*"} else {" "};
                print!("{}{}", self.trees[index], visible);
            }
            print!("\n");
        }
    }

    fn score_tree_1d(&self, idx: (usize, usize), dir: Step) -> usize {
        let mut score: usize = 0;
        let mut lidx = idx; 
        let tree = self.trees[idx.0*self.ncols+idx.1];
        while lidx.0 > 0 && lidx.0 < self.nrows-1 && lidx.1 > 0 && lidx.1 < self.ncols-1 {
            score += 1;
            match dir {
                Step::Up => { lidx.0 -= 1; }
                Step::Down => { lidx.0 += 1; }
                Step::Left => { lidx.1 -= 1; }
                Step::Right => { lidx.1 += 1; }
            }
            let next_tree = self.trees[lidx.0*self.ncols+lidx.1];
            if next_tree >= tree {
                break;
            }
        }
        score
    }

    fn scenic_score(&mut self) -> usize {
        let mut max_score = 0;
        for row in 0..self.nrows {
            for col in 0..self.ncols {
                let score_up = self.score_tree_1d((row, col), Step::Up);
                let score_left = self.score_tree_1d((row, col), Step::Left);
                let score_down = self.score_tree_1d((row, col), Step::Down);
                let score_right = self.score_tree_1d((row, col), Step::Right);
                let score = score_up*score_left*score_right*score_down;
                if score > max_score {
                    max_score = score;
                }
            }
        }
        max_score
    }
}


pub fn compute(arg: &str) -> () {
    let data = std::fs::read_to_string(arg).unwrap();
    let mut forest: Forest = Forest::default();
    forest.parse(&data);
    forest.find_visible();
    println!("  Part1: {}", forest.count_visible());
    println!("  Part2: {}", forest.scenic_score());
}
