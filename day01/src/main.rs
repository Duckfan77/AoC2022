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

fn part1(text: &String) {
    println!(
        "{}\n",
        text.split("\n\n")
            .map(|block| {
                block
                    .lines()
                    .map(|x| x.parse::<i64>().unwrap())
                    .sum::<i64>()
            })
            .max()
            .unwrap()
    );
}

fn part2(text: &str) {
    let mut elves: Vec<i64> = text
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|x| x.parse::<i64>().unwrap())
                .sum::<i64>()
        })
        .collect::<Vec<_>>();
    elves.sort();

    println!("{}\n", elves[elves.len() - 3..].iter().sum::<i64>());
}
