use serde_json::Value;
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

struct Pair {
    p1: Packet,
    p2: Packet,
}

impl Pair {
    fn part1_right(&self) -> bool {
        self.p1 < self.p2
    }
}

enum Packet {
    Num(i64),
    List(Vec<Packet>),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Num(l0), Self::Num(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Num(l0), Self::List(_)) => Packet::List(vec![Packet::Num(*l0)]) == *other,
            (Self::List(_), Self::Num(r0)) => *self == Packet::List(vec![Packet::Num(*r0)]),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Packet::Num(i1), Packet::Num(i2)) => i1.partial_cmp(i2),
            (Packet::Num(i1), Packet::List(_)) => {
                Packet::List(vec![Packet::Num(*i1)]).partial_cmp(other)
            }
            (Packet::List(_), Packet::Num(i1)) => {
                self.partial_cmp(&Packet::List(vec![Packet::Num(*i1)]))
            }
            (Packet::List(l1), Packet::List(l2)) => l1.partial_cmp(l2),
        }
    }
}

impl Packet {
    fn from_val(v: &Value) -> Packet {
        match v {
            Value::Null => unimplemented!("Null hit"),
            Value::Bool(_) => unimplemented!("bool hit"),
            Value::Number(i) => Packet::Num(i.as_i64().unwrap()),
            Value::String(_) => unimplemented!("string hit"),
            Value::Array(a) => Packet::List(a.into_iter().map(|v1| Packet::from_val(v1)).collect()),
            Value::Object(_) => unimplemented!("object hit"),
        }
    }
}

fn part1(text: &String) {
    let pairs = text
        .split("\n\n")
        .map(|block| {
            let (l1, l2) = block.split_once("\n").unwrap();
            Pair {
                p1: Packet::from_val(&serde_json::from_str(l1).unwrap()),
                p2: Packet::from_val(&serde_json::from_str(l2).unwrap()),
            }
        })
        .collect::<Vec<Pair>>();

    println!(
        "{}",
        pairs
            .iter()
            .enumerate()
            .filter_map(|(i, p)| if p.part1_right() { Some(i + 1) } else { None })
            .sum::<usize>()
    );
}

fn part2(text: &String) {}
