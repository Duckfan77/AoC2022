use std::collections::HashSet;
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

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn parse(s: &str) -> (Self, i32) {
        let (d, i) = s.split_once(" ").unwrap();
        let i = i.parse().unwrap();
        match d {
            "L" => (Direction::Left, i),
            "U" => (Direction::Up, i),
            "D" => (Direction::Down, i),
            "R" => (Direction::Right, i),
            _ => unreachable!(),
        }
    }
}

struct Rope {
    head: (i64, i64),
    tail: (i64, i64),
}

impl Rope {
    fn new() -> Self {
        Self {
            head: (0, 0),
            tail: (0, 0),
        }
    }

    fn update_tail(&mut self) {
        let xdif = self.head.0 - self.tail.0;
        let ydif = self.head.1 - self.tail.1;

        if ydif.abs() < 2 && xdif.abs() < 2 {
            return; // early return, nothing to do.
        }

        if xdif == 0 {
            // move on y
            self.tail.1 += ydif / 2; // ydif must be +-2
        } else if ydif == 0 {
            self.tail.0 += xdif / 2; // xdif must be +-2
        } else {
            // move on both
            if xdif.abs() == 2 {
                // xdif.abs() = 2 and ydif.abs() = 1
                self.tail.0 += xdif / 2;
                self.tail.1 += ydif;
            } else {
                // xdif.abs() = 1 and ydif.abs() = 2
                self.tail.0 += xdif;
                self.tail.1 += ydif / 2;
            }
        }
    }

    fn move_head(&mut self, dir: Direction) -> (i64, i64) {
        match dir {
            Direction::Up => self.head.1 += 1,
            Direction::Left => self.head.0 -= 1,
            Direction::Down => self.head.1 -= 1,
            Direction::Right => self.head.0 += 1,
        }

        self.update_tail();

        self.tail
    }
}

fn part1(text: &String) {
    let mut set = HashSet::new();
    let mut rope = Rope::new();
    set.insert((0, 0));
    for line in text.lines() {
        let (d, i) = Direction::parse(line);
        for _ in 0..i {
            set.insert(rope.move_head(d));
        }
    }

    println!("{}", set.len())
}

fn part2(text: &String) {}
