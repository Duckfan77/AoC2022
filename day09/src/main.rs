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
    knots: Vec<(i64, i64)>,
}

impl Rope {
    fn new(len: usize) -> Self {
        Self {
            knots: vec![(0, 0); len],
        }
    }

    fn update_knot(&mut self, i: usize) {
        let xdif = self.knots[i - 1].0 - self.knots[i].0;
        let ydif = self.knots[i - 1].1 - self.knots[i].1;

        if ydif.abs() < 2 && xdif.abs() < 2 {
            return; // early return, nothing to do.
        }

        if xdif == 0 {
            // move on y
            self.knots[i].1 += ydif / 2; // ydif must be +-2
        } else if ydif == 0 {
            self.knots[i].0 += xdif / 2; // xdif must be +-2
        } else {
            // move on both
            if xdif.abs() == 2 && ydif.abs() == 2 {
                self.knots[i].0 += xdif / 2;
                self.knots[i].1 += ydif / 2;
            } else if xdif.abs() == 2 {
                // xdif.abs() = 2 and ydif.abs() = 1
                self.knots[i].0 += xdif / 2;
                self.knots[i].1 += ydif;
            } else {
                // xdif.abs() = 1 and ydif.abs() = 2
                self.knots[i].0 += xdif;
                self.knots[i].1 += ydif / 2;
            }
        }
    }

    fn update_tail(&mut self) {
        for i in 1..self.knots.len() {
            self.update_knot(i);
        }
    }

    fn move_head(&mut self, dir: Direction) -> (i64, i64) {
        match dir {
            Direction::Up => self.knots[0].1 += 1,
            Direction::Left => self.knots[0].0 -= 1,
            Direction::Down => self.knots[0].1 -= 1,
            Direction::Right => self.knots[0].0 += 1,
        }

        self.update_tail();

        *self.knots.last().unwrap()
    }
}

fn part1(text: &String) {
    let mut set = HashSet::new();
    let mut rope = Rope::new(2);
    set.insert((0, 0));
    for line in text.lines() {
        let (d, i) = Direction::parse(line);
        for _ in 0..i {
            set.insert(rope.move_head(d));
        }
    }

    println!("{}", set.len())
}

fn part2(text: &String) {
    let mut set = HashSet::new();
    let mut rope = Rope::new(10);
    set.insert((0, 0));
    for line in text.lines() {
        let (d, i) = Direction::parse(line);
        for _ in 0..i {
            set.insert(rope.move_head(d));
        }
    }

    println!("{}", set.len())
}
