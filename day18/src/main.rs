use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Add;

fn main() {
    let mut file: File = File::open("input").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    println!("Part 1:");
    part1(&text);

    println!("\nPart 2:");
    part2(&text);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Add for Point3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Point3 {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn from_str(s: &str) -> Self {
        let mut i = s.split(",");
        Self {
            x: i.next().unwrap().parse().unwrap(),
            y: i.next().unwrap().parse().unwrap(),
            z: i.next().unwrap().parse().unwrap(),
        }
    }
}

fn adj_to(pt: &Point3, map: &HashSet<Point3>) -> u64 {
    let adj_map = [
        Point3::new(-1, 0, 0),
        Point3::new(1, 0, 0),
        Point3::new(0, -1, 0),
        Point3::new(0, 1, 0),
        Point3::new(0, 0, -1),
        Point3::new(0, 0, 1),
    ];

    6 - adj_map
        .iter()
        .map(|a| *a + *pt)
        .filter(|p| map.get(p).is_some())
        .count() as u64
}

fn part1(text: &String) {
    let map: HashSet<Point3> = text.lines().map(|line| Point3::from_str(line)).collect();
    println!("{}", map.iter().map(|pt| adj_to(pt, &map)).sum::<u64>());
}

fn part2(text: &String) {}
