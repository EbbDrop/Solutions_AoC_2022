#![feature(int_roundings)]

use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone)]
struct Recipie {
    needed_ore: i16,
    needed_clay: i16,
    needed_obsidian: i16,
}

impl Recipie {
    #[must_use]
    fn max(&self, other: &Self) -> Self {
        Self {
            needed_ore: self.needed_ore.max(other.needed_ore),
            needed_clay: self.needed_clay.max(other.needed_clay),
            needed_obsidian: self.needed_obsidian.max(other.needed_obsidian),
        }
    }
}

#[derive(Debug, Clone)]
struct Blueprint {
    ore_robot: Recipie,
    clay_robot: Recipie,
    obsidian_robot: Recipie,
    geode_robot: Recipie,
    max_robot: Recipie,
}

impl FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Blueprint ").ok_or(())?;
        let (_, rest) = s.split_once(": Each ore robot costs ").ok_or(())?;
        let (ore_for_ore, rest) = rest.split_once(" ore. Each clay robot costs ").ok_or(())?;
        let (ore_for_clay, rest) = rest
            .split_once(" ore. Each obsidian robot costs ")
            .ok_or(())?;
        let (ore_for_obsidian, rest) = rest.split_once(" ore and ").ok_or(())?;
        let (clay_for_obsidian, rest) = rest
            .split_once(" clay. Each geode robot costs ")
            .ok_or(())?;
        let (ore_for_geode, rest) = rest.split_once(" ore and ").ok_or(())?;
        let obsidian_for_geode = rest.strip_suffix(" obsidian.").ok_or(())?;

        let ore_recipie = Recipie {
            needed_ore: ore_for_ore.parse().map_err(|_| ())?,
            needed_clay: 0,
            needed_obsidian: 0,
        };

        let clay_recipie = Recipie {
            needed_ore: ore_for_clay.parse().map_err(|_| ())?,
            needed_clay: 0,
            needed_obsidian: 0,
        };

        let obsidian_recipie = Recipie {
            needed_ore: ore_for_obsidian.parse().map_err(|_| ())?,
            needed_clay: clay_for_obsidian.parse().map_err(|_| ())?,
            needed_obsidian: 0,
        };

        let geode_recipie = Recipie {
            needed_ore: ore_for_geode.parse().map_err(|_| ())?,
            needed_clay: 0,
            needed_obsidian: obsidian_for_geode.parse().map_err(|_| ())?,
        };
        let max_robot = ore_recipie
            .max(&clay_recipie)
            .max(&obsidian_recipie)
            .max(&geode_recipie);

        Ok(Blueprint {
            ore_robot: ore_recipie,
            clay_robot: clay_recipie,
            obsidian_robot: obsidian_recipie,
            geode_robot: geode_recipie,
            max_robot,
        })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Resource {
    amount: i16,
    speed: i16,
}

impl Resource {
    fn update(&mut self, t: i16) {
        self.amount += self.speed * t;
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Current {
    time_left: i16,
    ore_resources: Resource,
    clay_resources: Resource,
    obsidian_resources: Resource,
    geode_resources: Resource,
}

impl Current {
    fn update(&mut self, t: i16) {
        self.time_left -= t;
        self.ore_resources.update(t);
        self.clay_resources.update(t);
        self.obsidian_resources.update(t);
        self.geode_resources.update(t);
    }

    fn sub_recipie(&mut self, recipie: &Recipie) {
        self.ore_resources.amount -= recipie.needed_ore;
        self.clay_resources.amount -= recipie.needed_clay;
        self.obsidian_resources.amount -= recipie.needed_obsidian;
    }
}

type Ans = i16;

fn run(current: Current, blueprint: &Blueprint, mem: &mut HashMap<Current, Ans>) -> Ans {
    if current.time_left == 0 {
        return current.geode_resources.amount;
    }
    assert!(current.time_left > 0);
    if let Some(a) = mem.get(&current) {
        return *a;
    }

    let try_create = |recipie: &Recipie| {
        let mut max_time_needed = 1;

        let needed_ore = recipie.needed_ore - current.ore_resources.amount;
        if needed_ore <= 0 {
            // alreade have enough ore
        } else if current.ore_resources.speed != 0 {
            max_time_needed =
                max_time_needed.max(needed_ore.div_ceil(current.ore_resources.speed) + 1);
        } else {
            return None;
        }

        let needed_clay = recipie.needed_clay - current.clay_resources.amount;
        if needed_clay <= 0 {
            // alreade have enough clay
        } else if current.clay_resources.speed != 0 {
            max_time_needed =
                max_time_needed.max(needed_clay.div_ceil(current.clay_resources.speed) + 1);
        } else {
            return None;
        }

        let needed_obsidian = recipie.needed_obsidian - current.obsidian_resources.amount;
        if needed_obsidian <= 0 {
            // alreade have enough obsidian
        } else if current.obsidian_resources.speed != 0 {
            max_time_needed =
                max_time_needed.max(needed_obsidian.div_ceil(current.obsidian_resources.speed) + 1);
        } else {
            return None;
        }

        if max_time_needed > current.time_left {
            return None;
        }

        let mut new_current = current.clone();
        new_current.update(max_time_needed);
        new_current.sub_recipie(recipie);
        Some(new_current)
    };

    let mut max_geodes = current.geode_resources.amount;

    if current.ore_resources.speed < blueprint.max_robot.needed_ore {
        if let Some(mut new_current) = try_create(&blueprint.ore_robot) {
            new_current.ore_resources.speed += 1;
            max_geodes = max_geodes.max(run(new_current.clone(), blueprint, mem));
        }
    }
    if current.clay_resources.speed < blueprint.max_robot.needed_clay {
        if let Some(mut new_current) = try_create(&blueprint.clay_robot) {
            new_current.clay_resources.speed += 1;
            max_geodes = max_geodes.max(run(new_current.clone(), blueprint, mem));
        }
    }
    if current.obsidian_resources.speed < blueprint.max_robot.needed_obsidian {
        if let Some(mut new_current) = try_create(&blueprint.obsidian_robot) {
            new_current.obsidian_resources.speed += 1;
            max_geodes = max_geodes.max(run(new_current.clone(), blueprint, mem));
        }
    }
    if let Some(mut new_current) = try_create(&blueprint.geode_robot) {
        new_current.geode_resources.speed += 1;
        max_geodes = max_geodes.max(run(new_current.clone(), blueprint, mem));
    }

    mem.insert(current, max_geodes);
    max_geodes
}

fn main() {
    // let input = include_str!("./example.txt");
    let input = include_str!("./input.txt");

    let mut blueprints: Vec<Blueprint> = Vec::new();
    for l in input.lines().take(3) {
        blueprints.push(l.parse().unwrap());
    }

    let current = Current {
        time_left: 32,
        ore_resources: Resource {
            amount: 0,
            speed: 1,
        },
        ..Default::default()
    };

    let mut total = 1;
    for (_, blueprint) in blueprints.iter().enumerate() {
        let mut mem = HashMap::new();
        total *= dbg!(run(current.clone(), blueprint, &mut mem));
    }
    dbg!(total);
}
