use indextree::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file: File = File::open("input").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    println!("Part 1:");
    part1(&text);

    println!("\nPart 2:");
    part2(&text);
}

#[derive(Debug)]
enum Line {
    Cd { dirname: String },
    Ls { contents: Vec<Line> },
    Dir { name: String },
    File { name: String, size: u64 },
}

impl Line {
    pub fn parse_block(mut block: &str) -> Self {
        block = block.trim();
        if block.starts_with("cd") {
            let mut name = block.strip_prefix("cd ").unwrap();
            name = name.trim();
            Line::Cd {
                dirname: name.to_string(),
            }
        } else {
            let mut lines = block.lines();
            lines.next(); // discard the ls line
            let mut contents = Vec::with_capacity(lines.size_hint().0);
            for line in lines {
                contents.push(Self::parse_ls_line(line))
            }
            Line::Ls { contents }
        }
    }

    fn parse_ls_line(line: &str) -> Self {
        match line.strip_prefix("dir ") {
            Some(mut name) => {
                name = name.trim();
                Line::Dir {
                    name: name.to_string(),
                }
            }
            None => {
                let (size, name) = line.split_once(" ").unwrap();
                Line::File {
                    size: size.parse().unwrap(),
                    name: name.to_string(),
                }
            }
        }
    }
}

#[derive(Debug)]
enum DirTreeNode {
    Dir { name: String, size: Option<u64> },
    File { size: u64, _name: String },
}

fn get_size(node: NodeId, tree: &mut Arena<DirTreeNode>) -> u64 {
    let sum = node
        .children(tree)
        .collect::<Vec<_>>()
        .iter()
        .map(|c| get_size(*c, tree))
        .sum();

    {
        match tree.get_mut(node).unwrap().get_mut() {
            DirTreeNode::Dir { name: _, size } => size.replace(sum),
            DirTreeNode::File { size, _name: _ } => return *size,
        };
    }

    sum
}

fn add_content(ls: Vec<Line>, curdir: NodeId, tree: &mut Arena<DirTreeNode>) {
    for line in ls {
        let node = match line {
            Line::Cd { dirname: _ } => unreachable!(),
            Line::Ls { contents: _ } => unreachable!(),
            Line::Dir { name } => DirTreeNode::Dir {
                name: name,
                size: None,
            },
            Line::File { name, size } => DirTreeNode::File { size, _name: name },
        };
        let nodeid = tree.new_node(node);
        curdir.append(nodeid, tree);
    }
}

fn find_dir(dirname: String, curdir: NodeId, tree: &Arena<DirTreeNode>) -> NodeId {
    if dirname == ".." {
        tree.get(curdir).unwrap().parent().unwrap_or(curdir)
    } else if dirname == "/" {
        curdir
    } else {
        for child in curdir.children(tree) {
            match tree.get(child).unwrap().get() {
                DirTreeNode::Dir { name, size: _ } => {
                    if dirname == *name {
                        return child;
                    }
                }
                DirTreeNode::File { size: _, _name: _ } => (),
            }
        }

        unreachable!("dir not found: {}", dirname)
    }
}

fn part1(text: &String) {
    let mut tree = Arena::new();
    let root = tree.new_node(DirTreeNode::Dir {
        name: "/".to_string(),
        size: None,
    });

    let mut curdir = root;

    for block in text.split("$") {
        let line = Line::parse_block(block);
        match line {
            Line::Cd { dirname } => curdir = find_dir(dirname, curdir, &tree),
            Line::Ls { contents } => add_content(contents, curdir, &mut tree),
            Line::Dir { name: _ } => unreachable!(),
            Line::File { name: _, size: _ } => unreachable!(),
        }
    }

    get_size(root, &mut tree); // populate all sizes for directories.

    let mut total = 0;
    for node in root.descendants(&tree) {
        total += match tree.get(node).unwrap().get() {
            DirTreeNode::Dir { name: _, size } => {
                if size.unwrap() <= 100000 {
                    size.unwrap()
                } else {
                    0
                }
            }
            DirTreeNode::File { size: _, _name: _ } => 0,
        }
    }

    println!("{}", total)
}

fn part2(text: &String) {
    let mut tree = Arena::new();
    let root = tree.new_node(DirTreeNode::Dir {
        name: "/".to_string(),
        size: None,
    });

    let mut curdir = root;

    for block in text.split("$") {
        let line = Line::parse_block(block);
        match line {
            Line::Cd { dirname } => curdir = find_dir(dirname, curdir, &tree),
            Line::Ls { contents } => add_content(contents, curdir, &mut tree),
            Line::Dir { name: _ } => unreachable!(),
            Line::File { name: _, size: _ } => unreachable!(),
        }
    }

    get_size(root, &mut tree); // populate all sizes for directories.

    let total = 70000000;
    let need = 30000000;
    let current = total
        - match tree.get(root).unwrap().get() {
            DirTreeNode::Dir { name: _, size } => size.unwrap(),
            DirTreeNode::File { size: _, _name: _ } => unreachable!(),
        };

    println!(
        "{}",
        root.descendants(&tree)
            .map(|nid| tree.get(nid).unwrap().get())
            .filter_map(|dirnode| match dirnode {
                DirTreeNode::Dir { name: _, size } => {
                    if (size.unwrap() + current) > need {
                        Some(size.unwrap())
                    } else {
                        None
                    }
                }
                DirTreeNode::File { size: _, _name: _ } => None,
            })
            .min()
            .unwrap()
    );
}
