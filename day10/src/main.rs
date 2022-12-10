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

enum Op {
    Noop,
    Add(i64),
}

impl Op {
    fn parse(s: &str) -> Self {
        if s == "noop" {
            Op::Noop
        } else {
            Op::Add(s.split_once(" ").unwrap().1.parse().unwrap())
        }
    }
}

fn update(i: i64, x: i64) -> i64 {
    if i % 40 == 20 {
        i * x
    } else {
        0
    }
}

fn part1(text: &String) {
    let mut i = 1;
    let mut x = 1;
    let mut total = 0;
    for op in text.lines().map(|line| Op::parse(line)) {
        match op {
            Op::Noop => i += 1,
            Op::Add(v) => {
                //cycle 1
                i += 1;
                total += update(i, x);
                // cycle 2
                i += 1;
                x += v;
            }
        }
        total += update(i, x);
    }
    println!("{}", total);
}

fn part2(text: &String) {}
