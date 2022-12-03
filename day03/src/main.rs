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

fn val(c: char) -> i64 {
    let i = c as i64;
    match c {
        'a'..='z' => i - 96,
        'A'..='Z' => i - 64 + 26,
        '.' => 0,
        _ => unreachable!(),
    }
}

fn part1(text: &String) {
    println!(
        "{}",
        text.lines()
            .map(|line| {
                let (f, l) = line.split_at(line.len() / 2);
                let f = f.chars().collect::<Vec<_>>();
                val(l.chars().find(|c| f.contains(c)).unwrap_or('.'))
            })
            .sum::<i64>()
    );
}

fn part2(_text: &String) {}
