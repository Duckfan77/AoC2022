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

#[derive(Debug)]
struct Range {
    l: i32,
    r: i32,
}

impl From<&str> for Range {
    fn from(item: &str) -> Self {
        let (strl, strr) = item.split_once("-").expect("item should have dash");
        Self {
            l: strl.parse().unwrap(),
            r: strr.parse().unwrap(),
        }
    }
}

fn contains(r1: Range, r2: Range) -> bool {
    (r1.l <= r2.l && r1.r >= r2.r) || (r2.l <= r1.l && r2.r >= r1.r)
}

fn part1(text: &String) {
    println!(
        "{}",
        text.lines()
            .map(|l| {
                let (l, r) = l.split_once(",").unwrap();
                (l, r)
            })
            .filter(|(l, r)| contains((*l).into(), (*r).into()))
            .count()
    );
}

fn part2(_text: &String) {}
