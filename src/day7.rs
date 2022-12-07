#[derive(Debug)]
struct File {
    size: usize,
    name: String,
}

impl<'a> std::iter::Sum<&'a Self> for File {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self { size: 0, name: "".to_string() }, |a, b| Self {
            size: a.size + b.size,
            name: "".to_string(),
        })
    }
}

#[derive(Debug)]
struct Directory {
    idx: usize,
    files: Vec<File>,
    parent: Option<usize>,
    children: Vec<usize>,
    name: String,
}

#[derive(Debug, Default)]
struct DirectoryArena {
    arena: Vec<Directory>,
    sizes: Vec<usize>,
}

impl Directory {
    fn new(idx: usize, name: &str) -> Self {
        Self {
            idx,
            files: vec![],
            parent: None,
            children: vec![],
            name: name.to_string(),
        }
    }

    fn sum(&self) -> usize {
        let sum_file: File = self.files.iter().sum();
        sum_file.size
    }
}

impl DirectoryArena {
    fn node(&mut self, name: &str) -> usize {
        let idx = self.arena.len();
        self.arena.push(Directory::new(idx, name));
        self.sizes.push(0);
        idx
    }

    fn sum_all(&mut self) -> () {
        let mut size;
        for directory in self.arena.iter().rev() {
            size = directory.sum();
            for child in directory.children.iter() {
                if self.sizes[*child] == 0 {
                    size += self.arena[*child].sum();
                } else {
                    size += self.sizes[*child];
                }
            }
            self.sizes[directory.idx] = size;
        }
    }

    fn count_directory(&self, min_size: &usize) -> usize {
        let mut count = 0;
        for size in self.sizes.iter() {
            if size < min_size {
                count += size;
            }
        }
        count
    }

    fn change_directory(&mut self, dir: &str, cd: &mut usize) {
        match dir {
            ".." => { *cd = self.arena[*cd].parent.unwrap(); },
            &_ => {
                for child in self.arena[*cd].children.iter() {
                    if self.arena[*child].name == dir {
                        *cd = *child;
                        break;
                    }
                }
            },
        }
    }
}

pub fn compute(arg: &str) -> () {
    let data = std::fs::read_to_string(arg).unwrap();
    let mut arena: DirectoryArena = DirectoryArena::default();
    let mut cd = arena.node("/");
    for line in data.lines() {
        match line {
            x if x.contains("$ ls") => { continue; }
            x if x.contains("$ cd") => {
                let dir = x.split(" ").collect::<Vec<&str>>()[2];
                arena.change_directory(dir, &mut cd); }
            x if x.contains("dir") => { 
                let child = add_directory(x, &mut arena);
                arena.arena[cd].children.push(child);
                arena.arena[child].parent = Some(cd);
            }
            &_ => { add_file(line, &mut arena.arena[cd]); }
        }
    }

    arena.sum_all();
    println!("  Part1: {}", arena.count_directory(&100000));

    let system_size: usize = 70000000;
    let update_size: usize = 30000000;
    let used_size = arena.sizes[0];
    let unused_size = system_size - used_size;
    let needed_space = update_size - unused_size;

    let mut sizes = arena.sizes;
    sizes.sort();
    match sizes.binary_search(&needed_space) {
        Ok(_pos) => {},
        Err(pos) => println!("  Part2: {}", sizes[pos]),
    }
}

fn add_directory(arg: &str, arena: &mut DirectoryArena) -> usize {
    let tokens: Vec<&str> = arg.split(" ").collect();
    let name = tokens[1];
    arena.node(name)
}

fn add_file(arg: &str, dir: &mut Directory) -> () {
    let tokens: Vec<&str> = arg.split(" ").collect();
    let size = tokens[0].parse::<usize>().unwrap();
    let name = tokens[1];
    dir.files.push(File{size: size, name: name.to_string()});
}
