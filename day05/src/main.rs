use itertools::Itertools;
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

fn get_start() -> Vec<Vec<char>> {
    vec![
        vec![], // 1 indexed, just do this to make it simpler
        vec!['F', 'C', 'P', 'G', 'Q', 'R'],
        vec!['W', 'T', 'C', 'P'],
        vec!['B', 'H', 'P', 'M', 'C'],
        vec!['L', 'T', 'Q', 'S', 'M', 'P', 'R'],
        vec!['P', 'H', 'J', 'Z', 'V', 'G', 'N'],
        vec!['D', 'P', 'J'],
        vec!['L', 'G', 'P', 'Z', 'F', 'J', 'T', 'R'],
        vec!['N', 'L', 'H', 'C', 'F', 'P', 'T', 'J'],
        vec!['G', 'V', 'Z', 'Q', 'H', 'T', 'C', 'W'],
    ]
}

fn move_crate(stacks: &mut Vec<Vec<char>>, start: usize, end: usize, count: usize) {
    for _ in 0..count {
        let c = stacks[start].pop().expect("popped from empty stack");
        stacks[end].push(c);
    }
}

fn parse_line(line: &str) -> (usize, usize, usize) {
    let parts = line.split_whitespace().collect::<Vec<_>>();
    (
        parts[1].parse().unwrap(),
        parts[3].parse().unwrap(),
        parts[5].parse().unwrap(),
    )
}

fn move_crates_group(stacks: &mut Vec<Vec<char>>, start: usize, end: usize, count: usize) {
    let mut temp = Vec::new();
    for _ in 0..count {
        temp.push(stacks[start].pop().expect("popped from empty stack"));
    }
    for _ in 0..count {
        stacks[end].push(temp.pop().unwrap());
    }
}

fn part1(text: &String) {
    let mut stacks = get_start();
    for line in text.lines() {
        let (count, start, end) = parse_line(line);
        move_crate(&mut stacks, start, end, count);
    }
    for stack in stacks {
        print!("{}", stack.last().unwrap_or(&' '));
    }
    println!();
}

fn part2(text: &String) {
    let mut stacks = get_start();
    for line in text.lines() {
        let (count, start, end) = parse_line(line);
        move_crates_group(&mut stacks, start, end, count);
    }
    for stack in stacks {
        print!("{}", stack.last().unwrap_or(&' '));
    }
    println!();
}
