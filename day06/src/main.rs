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

fn part1(text: &String) {
    println!(
        "{}",
        text.char_indices()
            .tuple_windows()
            .find(|(a, b, c, d)| {
                a.1 != b.1 && a.1 != c.1 && a.1 != d.1 && b.1 != c.1 && b.1 != d.1 && c.1 != d.1
            })
            .unwrap()
            .3
             .0
            + 1
    );
}

fn part2(text: &String) {
    let chars = text.char_indices().collect::<Vec<_>>();
    println!(
        "{}",
        chars
            .windows(14)
            .find(|x| {
                for i in 0..x.len() {
                    for j in i + 1..x.len() {
                        if x[i].1 == x[j].1 {
                            return false;
                        }
                    }
                }
                true
            })
            .unwrap()[13]
            .0
            + 1
    );
}
