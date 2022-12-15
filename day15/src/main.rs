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

static Y: i64 = 2000000;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn manhattan_dist(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn _points_at_dist(dist: u64) -> impl Iterator<Item = Point> {
    (0..=dist as i64).into_iter().flat_map(move |x| {
        let y = dist as i64 - x;
        [
            Point::new(x, y),
            Point::new(-x, y),
            Point::new(x, -y),
            Point::new(-x, -y),
        ]
        .into_iter()
    })
}

fn _points_upto_dist(dist: u64) -> impl Iterator<Item = Point> {
    (0..=dist).into_iter().flat_map(|d| _points_at_dist(d))
}

fn _points_upto_dist_from(p: &Point, dist: u64) -> impl Iterator<Item = Point> {
    let pclone = p.clone();
    _points_upto_dist(dist).map(move |dp| pclone + dp)
}

fn points_at_y(p: &Point, dist: u64) -> impl Iterator<Item = Point> {
    let x = p.x;
    let y = p.y;
    let yoff = Y.abs_diff(y) as i64;
    (yoff..=dist as i64).into_iter().flat_map(move |ndist| {
        let xoff = ndist - yoff;
        vec![Point::new(x + xoff, Y), Point::new(x - xoff, Y)].into_iter()
    })
}

fn part1(text: &String) {
    let modified_input = text
        .replace("Sensor at x=", "")
        .replace(" y=", "")
        .replace(": closest beacon is at x=", ":");

    let mut blocked: HashSet<Point> = HashSet::new();
    let mut beacons: HashSet<Point> = HashSet::new();

    for (i, line) in modified_input.lines().enumerate() {
        println!("line: {}", i + 1);

        let (p1, p2) = line.split_once(":").unwrap();
        let (x1, y1) = p1.split_once(",").unwrap();
        let (x2, y2) = p2.split_once(",").unwrap();
        let source = Point::new(x1.parse().unwrap(), y1.parse().unwrap());
        let beacon = Point::new(x2.parse().unwrap(), y2.parse().unwrap());
        blocked.extend(points_at_y(&source, source.manhattan_dist(&beacon)));
        if beacon.y == Y {
            beacons.insert(beacon);
        }
    }

    //println!("{:?}", blocked);

    println!(
        "{}",
        blocked.iter().filter(|p| p.y == Y).count() - beacons.len()
    )

    // 4687825
}

fn part2(text: &String) {}