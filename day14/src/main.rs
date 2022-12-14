use itertools::Itertools;
use std::collections::HashMap;
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

enum Tile {
    Rock,
    ActiveSand,
    RestSand,
}

struct Grid {
    bottom_edge: usize,
    source: (usize, usize),
    active: (usize, usize),
    grid: HashMap<(usize, usize), Tile>,
}

#[derive(PartialEq, Eq)]
enum State {
    Stepped,
    Rest,
    Void,
}

impl Grid {
    fn step(&mut self) -> State {
        if self.active.1 > self.bottom_edge + 4 {
            return State::Void;
        }

        let down1 = (self.active.0, self.active.1 + 1);
        match self.grid.get(&down1) {
            None => {
                self.grid.remove(&self.active);
                self.grid.insert(down1, Tile::ActiveSand);
                self.active = down1;
                return State::Stepped;
            }
            Some(Tile::RestSand) | Some(Tile::Rock) => (),
            Some(Tile::ActiveSand) => unreachable!(),
        };

        let ld1 = (self.active.0 - 1, self.active.1 + 1);
        match self.grid.get(&ld1) {
            None => {
                self.grid.remove(&self.active);
                self.grid.insert(ld1, Tile::ActiveSand);
                self.active = ld1;
                return State::Stepped;
            }
            Some(Tile::RestSand) | Some(Tile::Rock) => (),
            Some(Tile::ActiveSand) => unreachable!(),
        };

        let rd1 = (self.active.0 + 1, self.active.1 + 1);
        match self.grid.get(&rd1) {
            None => {
                self.grid.remove(&self.active);
                self.grid.insert(rd1, Tile::ActiveSand);
                self.active = rd1;
                return State::Stepped;
            }
            Some(Tile::RestSand) | Some(Tile::Rock) => (),
            Some(Tile::ActiveSand) => unreachable!(),
        };

        self.grid.insert(self.active, Tile::RestSand);
        State::Rest
    }

    fn run(&mut self) -> State {
        let mut state = State::Stepped;
        self.grid.insert(self.source, Tile::ActiveSand);
        self.active = self.source;

        while state == State::Stepped {
            state = self.step();
        }

        state
    }

    fn count_spawns(&mut self) -> usize {
        let mut state = State::Rest;
        let mut i = 0;
        while state != State::Void {
            state = self.run();
            i = i + 1;
        }

        i - 1
    }

    fn step_floor(&mut self) -> State {
        if self.active.1 == self.bottom_edge + 1 {
            self.grid.insert(self.active, Tile::RestSand);
            return State::Rest;
        }
        self.step()
    }

    fn run_floor(&mut self) -> State {
        let mut state = State::Stepped;
        self.grid.insert(self.source, Tile::ActiveSand);
        self.active = self.source;

        while state == State::Stepped {
            state = self.step_floor();
        }

        if self.active == self.source {
            State::Void
        } else {
            state
        }
    }

    fn count_spawns_floor(&mut self) -> usize {
        let mut state = State::Rest;
        let mut i = 0;
        while state != State::Void {
            state = self.run_floor();
            i = i + 1;
        }

        i
    }

    fn new(file: &str) -> Self {
        let mut grid = HashMap::new();
        let mut bottom = 0;

        for line in file.lines() {
            for ((x1, y1), (x2, y2)) in line
                .split(" -> ")
                .map(|s| {
                    s.split_once(",")
                        .and_then(|(s1, s2)| {
                            Some((s1.parse::<usize>().unwrap(), s2.parse::<usize>().unwrap()))
                        })
                        .unwrap()
                })
                .tuple_windows()
            {
                if y1 > bottom {
                    bottom = y1;
                }
                if y2 > bottom {
                    bottom = y2;
                }

                if x1 == x2 {
                    for y in y1.min(y2)..=y1.max(y2) {
                        grid.insert((x1, y), Tile::Rock);
                    }
                } else if y1 == y2 {
                    for x in x1.min(x2)..=x1.max(x2) {
                        grid.insert((x, y1), Tile::Rock);
                    }
                }
            }
        }

        Self {
            bottom_edge: bottom,
            source: (500, 0),
            active: (500, 0),
            grid,
        }
    }
}

fn part1(text: &String) {
    let mut grid = Grid::new(text);
    println!("{}", grid.count_spawns());
}

fn part2(text: &String) {
    let mut grid = Grid::new(text);
    println!("{}", grid.count_spawns_floor());
}
