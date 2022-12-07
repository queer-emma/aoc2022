use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{
        Rc,
        Weak,
    },
};

#[derive(Debug)]
enum LsEntry {
    Dir { name: String },
    File { name: String, file_size: u64 },
}

#[derive(Debug)]
enum Command {
    Ls { files: Vec<LsEntry> },
    Cd { path: String },
}

enum FsNode {
    File {
        name: String,
        file_size: u64,
        parent: Rc<FsNode>,
    },
    Directory {
        name: String,
        files: RefCell<HashMap<String, Rc<FsNode>>>,
        parent: Option<Weak<FsNode>>,
        total_size: RefCell<u64>,
    },
}

impl FsNode {
    pub fn parent(&self) -> Rc<FsNode> {
        match self {
            FsNode::File { parent, .. } => parent.clone(),
            FsNode::Directory { parent, .. } => {
                parent
                    .as_ref()
                    .expect("no parent directory")
                    .upgrade()
                    .unwrap()
            }
        }
    }

    pub fn total_size(&self) -> u64 {
        match self {
            FsNode::File { file_size, .. } => *file_size,
            FsNode::Directory { total_size, .. } => *total_size.borrow(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            FsNode::File { name, .. } => name,
            FsNode::Directory { name, .. } => name,
        }
    }

    pub fn compute_total_size(&self) -> u64 {
        match self {
            FsNode::File { file_size, .. } => *file_size,
            FsNode::Directory {
                files, total_size, ..
            } => {
                let mut total = 0;

                for file in files.borrow().values() {
                    total += file.compute_total_size();
                }

                *total_size.borrow_mut() = total;
                total
            }
        }
    }

    pub fn find_directories_with_atmost_100000(&self) -> u64 {
        let mut total = 0;

        match self {
            FsNode::File { .. } => {}
            FsNode::Directory {
                files, total_size, ..
            } => {
                let size = *total_size.borrow();
                if size < 100000 {
                    total += size;
                }
                for file in files.borrow().values() {
                    total += file.find_directories_with_atmost_100000();
                }
            }
        }

        total
    }

    pub fn find_smallest_above(self: Rc<Self>, above_size: u64) -> Option<Rc<FsNode>> {
        match self.as_ref() {
            FsNode::File { .. } => {}
            FsNode::Directory {
                files, total_size, ..
            } => {
                let mut min_dir: Option<Rc<FsNode>> = None;

                for file in files.borrow().values() {
                    if let Some(smallest_dir) = file.clone().find_smallest_above(above_size) {
                        if let Some(min_dir) = &mut min_dir {
                            if smallest_dir.total_size() < min_dir.total_size() {
                                *min_dir = smallest_dir;
                            }
                        }
                        else {
                            min_dir = Some(smallest_dir)
                        }
                    }
                }

                if let Some(min_dir) = min_dir {
                    return Some(min_dir);
                }

                if *total_size.borrow() > above_size {
                    return Some(self.clone());
                }
            }
        }

        None
    }
}

fn build_fs_from_commands(commands: &[Command]) -> Rc<FsNode> {
    let root = Rc::new(FsNode::Directory {
        name: "/".to_owned(),
        files: RefCell::new(HashMap::new()),
        parent: None,
        total_size: RefCell::new(0),
    });
    let mut current = root.clone();

    for command in commands {
        match command {
            Command::Ls { files } => {
                match current.as_ref() {
                    FsNode::File { .. } => panic!("current is not a directory"),
                    FsNode::Directory {
                        files: dir_files, ..
                    } => {
                        let mut dir_files = dir_files.borrow_mut();
                        for entry in files {
                            match entry {
                                LsEntry::Dir { name } => {
                                    dir_files.insert(
                                        name.to_string(),
                                        Rc::new(FsNode::Directory {
                                            name: name.to_string(),
                                            files: RefCell::new(HashMap::new()),
                                            parent: Some(Rc::downgrade(&current)),
                                            total_size: RefCell::new(0),
                                        }),
                                    );
                                }
                                LsEntry::File { name, file_size } => {
                                    dir_files.insert(
                                        name.to_string(),
                                        Rc::new(FsNode::File {
                                            name: name.to_string(),
                                            file_size: *file_size,
                                            parent: current.clone(),
                                        }),
                                    );
                                }
                            }
                        }
                    }
                }
            }
            Command::Cd { path } => {
                if path == ".." {
                    current = current.parent();
                }
                else {
                    let new_current = match current.as_ref() {
                        FsNode::File { .. } => panic!("current is not a directory"),
                        FsNode::Directory { files, .. } => {
                            let mut files = files.borrow_mut();
                            if let Some(dir) = files.get(path) {
                                dir.clone()
                            }
                            else {
                                let dir = Rc::new(FsNode::Directory {
                                    name: path.to_string(),
                                    files: RefCell::new(HashMap::new()),
                                    parent: Some(Rc::downgrade(&current)),
                                    total_size: RefCell::new(0),
                                });
                                files.insert(path.to_string(), dir.clone());
                                dir
                            }
                        }
                    };
                    current = new_current;
                }
            }
        }
    }

    root
}

#[aoc_generator(day7)]
fn day7_input(input: &str) -> Rc<FsNode> {
    let mut lines = input.lines().peekable();
    let mut commands = vec![];

    while let Some(command) = lines.next() {
        let command = command.split_whitespace().collect::<Vec<_>>();
        assert_eq!(command[0], "$");

        match command[1] {
            "ls" => {
                let mut files = vec![];
                loop {
                    let Some(line) = lines.peek() else {break};
                    if line.starts_with("$") {
                        break;
                    }

                    let output = lines.next().unwrap().split_whitespace().collect::<Vec<_>>();
                    let name = output[1].to_string();

                    if output[0] == "dir" {
                        files.push(LsEntry::Dir { name });
                    }
                    else {
                        let file_size = output[0].parse().unwrap();
                        files.push(LsEntry::File { name, file_size })
                    }
                    //output.push();
                }
                commands.push(Command::Ls { files });
            }
            "cd" => {
                commands.push(Command::Cd {
                    path: command[2].to_string(),
                })
            }
            _ => panic!("invalid command: {}", command[1]),
        }
    }

    let fs = build_fs_from_commands(&commands);
    fs.compute_total_size();

    fs
}

#[aoc(day7, part1)]
fn day7_part1(fs: &Rc<FsNode>) -> u64 {
    fs.find_directories_with_atmost_100000()
}

#[aoc(day7, part2)]
fn day7_part2(fs: &Rc<FsNode>) -> u64 {
    let disk_size = 70000000;
    let total_size = fs.total_size();
    let free_space = disk_size - total_size;
    let need_to_free = 30000000 - free_space;

    println!("total_size: {}", total_size);
    println!("free_space: {}", free_space);
    println!("need_to_free: {}", need_to_free);

    let smallest_dir = fs.clone().find_smallest_above(need_to_free).unwrap();
    println!(
        "smallest dir: {} {}",
        smallest_dir.name(),
        smallest_dir.total_size()
    );
    smallest_dir.total_size()
}
