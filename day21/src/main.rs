use std::{collections::HashMap, fmt::format};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn calc(&self, first: i64, second: i64) -> i64 {
        match self {
            Op::Add => first + second,
            Op::Sub => first - second,
            Op::Mul => first * second,
            Op::Div => first / second,
        }
    }

    fn rev_know_first(&self, target: i64, first: i64) -> i64 {
        match self {
            Op::Add => target - first,
            Op::Sub => -target + first,
            Op::Mul => target / first,
            Op::Div => first / target,
        }
    }

    fn rev_know_second(&self, target: i64, second: i64) -> i64 {
        match self {
            Op::Add => target - second,
            Op::Sub => target + second,
            Op::Mul => target / second,
            Op::Div => target * second,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Calc {
    first: String,
    second: String,
    op: Op,
}

fn run(
    values: &mut HashMap<String, i64>,
    calcs: &HashMap<String, Calc>,
    name: &str,
) -> Option<i64> {
    if let Some(v) = values.get(name) {
        return Some(*v);
    }
    if let Some(c) = calcs.get(name) {
        let first = run(values, calcs, &c.first);
        let second = run(values, calcs, &c.second);

        if let (Some(first), Some(second)) = (first, second) {
            let v = c.op.calc(first, second);
            values.insert(name.to_owned(), v);
            return Some(v);
        }
    }
    None
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum CalcTree {
    Humn,
    FirstKnow {
        first: i64,
        second: Box<CalcTree>,
        op: Op,
    },
    SecondKnow {
        first: Box<CalcTree>,
        second: i64,
        op: Op,
    },
}

impl CalcTree {
    fn find_humn_value(&self, target: i64) -> i64 {
        match self {
            CalcTree::Humn => target,
            CalcTree::FirstKnow { first, second, op } => {
                let new_target = op.rev_know_first(target, *first);
                second.find_humn_value(new_target)
            }
            CalcTree::SecondKnow { first, second, op } => {
                let new_target = op.rev_know_second(target, *second);
                first.find_humn_value(new_target)
            }
        }
    }
}

fn to_tree(
    values: &mut HashMap<String, i64>,
    calcs: &HashMap<String, Calc>,
    name: &str,
) -> CalcTree {
    if name == "humn" {
        return CalcTree::Humn;
    }

    let c = calcs.get(name).unwrap();
    if let Some(first) = run(values, calcs, &c.first) {
        let second = Box::new(to_tree(values, calcs, &c.second));

        CalcTree::FirstKnow {
            first,
            second,
            op: c.op.clone(),
        }
    } else if let Some(second) = run(values, calcs, &c.second) {
        let first = Box::new(to_tree(values, calcs, &c.first));

        CalcTree::SecondKnow {
            first,
            second,
            op: c.op.clone(),
        }
    } else {
        panic!("Two humn dedected!")
    }
}

fn main() {
    // let input = include_str!("./example.txt");
    let input = include_str!("./input.txt");

    let mut values = HashMap::<String, i64>::new();
    let mut calcs = HashMap::<String, Calc>::new();

    for l in input.lines() {
        let (name, calc) = l.split_once(": ").unwrap();
        if name == "humn" {
            continue;
        }
        match calc.parse::<i64>() {
            Ok(v) => {
                values.insert(name.to_owned(), v);
            }
            Err(_) => {
                let (first, rest) = calc.split_once(' ').unwrap();
                let (op, second) = rest.split_once(' ').unwrap();

                let op = match op {
                    "+" => Op::Add,
                    "-" => Op::Sub,
                    "/" => Op::Div,
                    "*" => Op::Mul,
                    _ => panic!("{} not a valid operator", op),
                };

                calcs.insert(
                    name.to_owned(),
                    Calc {
                        first: first.to_owned(),
                        second: second.to_owned(),
                        op,
                    },
                );
            }
        }
    }

    let root_first = calcs.get("root").unwrap().first.clone();
    let root_second = calcs.get("root").unwrap().second.clone();

    let value_to_get = run(&mut values, &calcs, &root_second);
    dbg!(value_to_get);
    let tree = to_tree(&mut values, &calcs, &root_first);
    dbg!(tree.find_humn_value(value_to_get.unwrap()));
}
