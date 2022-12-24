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

#[derive(Clone, Copy)]
enum Op {
    Mul,
    Add,
    Sub,
    Div,
}

impl Op {
    fn run(&self, op1: i64, op2: i64) -> i64 {
        match self {
            Op::Mul => op1 * op2,
            Op::Add => op1 + op2,
            Op::Sub => op1 - op2,
            Op::Div => op1 / op2,
        }
    }
}

impl From<&str> for Op {
    fn from(value: &str) -> Self {
        match value {
            "*" => Self::Mul,
            "+" => Self::Add,
            "-" => Self::Sub,
            "/" => Self::Div,
            _ => panic!(),
        }
    }
}

#[derive(Clone)]
enum Monkey {
    Op {
        m1: String,
        m2: String,
        op: Op,
        value: Option<i64>,
    },
    Num {
        value: i64,
    },
}

impl Monkey {
    fn from_str(s: &str) -> (String, Self) {
        let (name, body) = s.split_once(": ").unwrap();
        let mut body = body.split_whitespace();
        let first = body.next().unwrap();
        let out = match first.parse() {
            Ok(i) => Self::Num { value: i },
            Err(_) => {
                let op: Op = body.next().unwrap().into();
                Self::Op {
                    m1: first.to_string(),
                    m2: body.next().unwrap().to_string(),
                    op,
                    value: None,
                }
            }
        };

        (name.to_string(), out)
    }

    fn get_val(&self) -> Option<i64> {
        match self {
            Monkey::Op {
                m1: _,
                m2: _,
                op: _,
                value,
            } => *value,
            Monkey::Num { value } => Some(*value),
        }
    }
}

struct MonkeySet {
    monkeys: HashMap<String, Monkey>,
}

impl MonkeySet {
    fn get_mnky_val(&mut self, name: &str) -> i64 {
        if let Some(i) = self.monkeys.get(name).unwrap().get_val() {
            i
        } else {
            // Monkey is an op, and doesn't have a value yet.
            let n1: String;
            let n2: String;
            let m_op: Op;
            match self.monkeys.get(name).unwrap() {
                Monkey::Op {
                    m1,
                    m2,
                    op,
                    value: _,
                } => {
                    m_op = *op;
                    n1 = m1.clone();
                    n2 = m2.clone()
                }
                Monkey::Num { value: _ } => unreachable!(),
            }

            let v1 = self.get_mnky_val(&n1);
            let v2 = self.get_mnky_val(&n2);

            let val = m_op.run(v1, v2);

            match self.monkeys.get_mut(name).unwrap() {
                Monkey::Op {
                    m1: _,
                    m2: _,
                    op: _,
                    value,
                } => value.replace(val),

                Monkey::Num { value: _ } => unreachable!(),
            };

            val
        }
    }

    fn from_str(s: &str) -> Self {
        Self {
            monkeys: s.lines().map(|line| Monkey::from_str(line)).collect(),
        }
    }
}

fn part1(text: &String) {
    let mut monkeys = MonkeySet::from_str(text);

    println!("{}", monkeys.get_mnky_val("root"));
}

fn part2(text: &String) {}
