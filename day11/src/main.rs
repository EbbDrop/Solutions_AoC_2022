type Item = u64;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
enum Operation {
    #[default]
    Square,
    Add(u64),
    Mul(u64),
}

#[derive(Debug, Clone, Default)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    div_test: u64,
    on_true: usize,
    on_false: usize,

    total_inspects: u64,
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}

fn main() {
    let mut lines = include_str!("./input.txt").lines();
    // let mut lines = include_str!("./example.txt").lines();

    let mut monkeys = Vec::new();

    while let Some(l) = lines.next() {
        if !l.starts_with("Monkey") {
            continue;
        }

        let mut monkey = Monkey::default();

        let items = lines.next().unwrap();
        let op = lines.next().unwrap();
        let test = lines.next().unwrap();
        let if_true = lines.next().unwrap();
        let if_false = lines.next().unwrap();

        let items = items.strip_prefix("  Starting items: ").unwrap();
        for item in items.split(", ") {
            monkey.items.push(item.parse().unwrap());
        }

        let op = op.strip_prefix("  Operation: new = old ").unwrap();
        if op.starts_with("+") {
            let amount = op.strip_prefix("+ ").unwrap();
            monkey.operation = Operation::Add(amount.parse().unwrap());
        } else {
            if op.ends_with("old") {
                monkey.operation = Operation::Square;
            } else {
                let amount = op.strip_prefix("* ").unwrap();
                monkey.operation = Operation::Mul(amount.parse().unwrap());
            }
        }

        let test = test.strip_prefix("  Test: divisible by ").unwrap();
        monkey.div_test = test.parse().unwrap();

        let if_true = if_true
            .strip_prefix("    If true: throw to monkey ")
            .unwrap();
        monkey.on_true = if_true.parse().unwrap();

        let if_false = if_false
            .strip_prefix("    If false: throw to monkey ")
            .unwrap();
        monkey.on_false = if_false.parse().unwrap();

        monkeys.push(monkey);
    }

    let mut lcm = 1;
    for monkey in &monkeys {
        lcm = num::integer::lcm(lcm, monkey.div_test);
    }
    dbg!(lcm);

    let rounds = 10000;

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let mut throws = Vec::new();

            let monkey = &mut monkeys[i];

            for item in monkey.items.iter() {
                let mut item = *item;
                item = match monkey.operation {
                    Operation::Square => mod_pow(item, 2, lcm),
                    Operation::Add(n) => item + n,
                    Operation::Mul(n) => item * n,
                } % lcm;

                monkey.total_inspects += 1;

                if item % monkey.div_test == 0 {
                    throws.push((item, monkey.on_true));
                } else {
                    throws.push((item, monkey.on_false));
                }
            }

            monkey.items.clear();
            for (item, monkey_i) in throws {
                monkeys[monkey_i].items.push(item);
            }
        }
    }

    let mut inspects = monkeys.iter().map(|m| m.total_inspects).collect::<Vec<_>>();
    inspects.sort();

    let mut inspects = inspects.iter().rev();
    dbg!(inspects.next().unwrap() * inspects.next().unwrap());
}
