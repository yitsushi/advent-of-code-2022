use std::str::FromStr;

use super::token::{Operation, Token};

#[derive(Debug, Default, Clone)]
pub struct Monkey {
    id: i64,
    items: Vec<i64>,
    operation: Operation,
    test: i64,
    if_true: i64,
    if_false: i64,
}

impl Monkey {
    pub fn process(&self, modi: i64, part1: bool) -> (Vec<(i64, i64)>, Self) {
        let targets = self.items.iter().map(|item| {
            let mut value = self.operation.clone().fold(*item, modi);
            if part1 {
                value /= 3;
            }
            let target = if value % self.test == 0 {
                self.if_true
            } else {
                self.if_false
            };
            (target, value)
        }).collect::<Vec<(i64, i64)>>();

        let mut new_self = self.clone();
        new_self.items = vec![];

        (targets, new_self)
    }

    pub fn modi(&self) -> i64 {
        self.test
    }

    pub fn push(self, value: i64) -> Self {
        let mut new_self = self;
        new_self.items.push(value);
        new_self
    }

    pub fn parse_line(&mut self, line: String) {
        if line.starts_with("Monkey") {
            let parts = line.split(' ').collect::<Vec<&str>>();
            self.id = parts[1].strip_suffix(':').unwrap().parse::<i64>().unwrap();

            return
        }

        if line.starts_with("  Starting items:") {
            let items: Vec<i64> = line.split(": ").last().unwrap()
                .split(", ")
                .map(|f| f.parse::<i64>().unwrap())
                .collect();
            self.items = items;

            return
        }

        if line.starts_with("  Operation:") {
            let ops: Vec<Token> = line.split(": ").last().unwrap()
                .split(' ').filter_map(|f| if let Ok(v) = Token::from_str(f) {
                    Some(v)
                } else {
                    None
                })
                .collect();
            self.operation = Operation::new(ops);

            return
        }

        if line.starts_with("  Test:") {
            let test: i64 = line.split(": ").last().unwrap()
                .split(' ').last().unwrap().parse::<i64>().unwrap();
            self.test = test;

            return
        }

        if line.starts_with("    If true:") {
            let answer: i64 = line.split(": ").last().unwrap()
                .split(' ').last().unwrap().parse::<i64>().unwrap();
            self.if_true = answer;

            return
        }

        if line.starts_with("    If false:") {
            let answer: i64 = line.split(": ").last().unwrap()
                .split(' ').last().unwrap().parse::<i64>().unwrap();
            self.if_false = answer;
        }
    }
}
