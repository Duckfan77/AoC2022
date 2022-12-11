use std::collections::VecDeque;
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
    Add(usize),
    Mul(usize),
    Square,
}

impl Op {
    fn apply(&self, old: usize) -> usize {
        match self {
            Op::Add(i) => old + i,
            Op::Mul(i) => old * i,
            Op::Square => old * old,
        }
    }
}

struct Monkey {
    items: VecDeque<usize>,
    op: Op,
    divtest: usize,
    true_i: usize,
    false_i: usize,
    count: usize,
}

impl Monkey {
    fn parse(s: &str) -> Self {
        let lines = s.lines().collect::<Vec<_>>();
        let items: VecDeque<usize> = lines[1]
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|i| i.parse().unwrap())
            .collect();
        let op_strings = lines[2].split_ascii_whitespace().collect::<Vec<_>>();
        let op = match op_strings[4] {
            "+" => Op::Add(op_strings[5].parse().unwrap()),
            "*" => {
                if op_strings[5] == "old" {
                    Op::Square
                } else {
                    Op::Mul(op_strings[5].parse().unwrap())
                }
            }
            _ => unreachable!("Invalid operation type: {}", op_strings[4]),
        };
        let div = lines[3].split_once("by ").unwrap().1.parse().unwrap();
        let true_i = lines[4].split_once("monkey ").unwrap().1.parse().unwrap();
        let false_i = lines[5].split_once("monkey ").unwrap().1.parse().unwrap();

        Self {
            items,
            op,
            divtest: div,
            true_i,
            false_i,
            count: 0,
        }
    }

    fn take_turn(&mut self) -> Vec<(usize, usize)> {
        self.count += self.items.len();

        self.items
            .drain(..)
            .map(|item| {
                let new = self.op.apply(item) / 3;
                if new % self.divtest == 0 {
                    (new, self.true_i)
                } else {
                    (new, self.false_i)
                }
            })
            .collect()
    }
}

fn part1(text: &String) {
    let mut monkeys = text
        .split("\n\n")
        .map(|block| Monkey::parse(block))
        .collect::<Vec<_>>();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let res = monkeys[i].take_turn();
            for (item, j) in res {
                monkeys[j].items.push_back(item);
            }
        }
    }

    let mut counts = monkeys
        .iter()
        .map(|monkey| monkey.count)
        .collect::<Vec<_>>();
    counts.sort();

    println!("{}", counts[counts.len() - 1] * counts[counts.len() - 2])
}

fn part2(text: &String) {}
