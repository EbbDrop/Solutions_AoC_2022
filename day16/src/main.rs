use kdam::Bar;
use kdam::BarExt;
use std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    ops::Add,
};

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct RoomID(u16);

impl std::fmt::Debug for RoomID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b = self.0 as u8;
        let a = (self.0 >> 8) as u8;
        write!(f, "{}{}", a as char, b as char)
    }
}

fn name_to_room_id(name: &str) -> RoomID {
    let id = match name.as_bytes() {
        [a, b] => (*a as u16) << 8 | (*b as u16),
        _ => panic!(
            "name needs to be 2 bytes long, not {} bytes long",
            name.len()
        ),
    };
    RoomID(id)
}

#[derive(Debug, Clone)]
struct Room {
    flow_rate: u64,
    leads_to: Vec<RoomID>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Current {
    room_me: RoomID,
    room_el: RoomID,
    move_time_left_me: u64,
    move_time_left_el: u64,

    time_left: u64,
    total_flow_rate: u64,
    claimd_rooms: BTreeSet<RoomID>,
}

impl Current {
    fn update(&mut self, rooms: &HashMap<RoomID, Room>) -> u64 {
        let used_time = self.move_time_left_me.min(self.move_time_left_el);

        let released = self.total_flow_rate * used_time;

        self.move_time_left_me -= used_time;
        self.move_time_left_el -= used_time;
        self.time_left -= used_time;

        if self.move_time_left_me == 0 {
            self.total_flow_rate += rooms.get(&self.room_me).unwrap().flow_rate;
        }
        if self.move_time_left_el == 0 {
            self.total_flow_rate += rooms.get(&self.room_el).unwrap().flow_rate;
        }
        released
    }

    fn update_non_null(&mut self, rooms: &HashMap<RoomID, Room>) -> u64 {
        let used_time = self.move_time_left_me.max(self.move_time_left_el);
        if used_time > self.time_left {
            return 0;
        }

        let released = self.total_flow_rate * used_time;
        self.time_left -= used_time;

        if self.move_time_left_me != 0 {
            self.move_time_left_me -= used_time;
            self.total_flow_rate += rooms.get(&self.room_me).unwrap().flow_rate;
        }
        if self.move_time_left_el != 0 {
            self.move_time_left_el -= used_time;
            self.total_flow_rate += rooms.get(&self.room_el).unwrap().flow_rate;
        }
        released
    }
}

#[derive(Debug, Default, Clone, Eq)]
struct Result {
    total: u64,
}

impl PartialEq for Result {
    fn eq(&self, other: &Self) -> bool {
        self.total.eq(&other.total)
    }
}

impl PartialOrd for Result {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Result {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total.cmp(&other.total)
    }
}

impl Add<Result> for Result {
    type Output = Result;

    fn add(self, rhs: Result) -> Self::Output {
        Self {
            total: self.total + rhs.total,
        }
    }
}

fn find_best(
    rooms: &HashMap<RoomID, Room>,
    best_paths: &HashMap<RoomID, HashMap<RoomID, u64>>,
    current: Current,
    saved: &mut HashMap<Current, Result>,
    first: bool,
) -> Result {
    debug_assert!(current.time_left > 0);

    if let Some(total) = saved.get(&current) {
        return total.clone();
    }

    let filter = |(&to_id, &length)| {
        let length = length + 1;
        if current.claimd_rooms.contains(&to_id) {
            return None;
        }
        if current.time_left < length {
            return None;
        }
        Some(Some((to_id, length)))
    };

    let me_options = if current.move_time_left_me == 0 {
        best_paths
            .get(&current.room_me)
            .unwrap()
            .iter()
            .filter_map(filter)
            .collect()
    } else {
        vec![None]
    };
    let el_options = if current.move_time_left_el == 0 {
        best_paths
            .get(&current.room_el)
            .unwrap()
            .iter()
            .filter_map(filter)
            .collect()
    } else {
        vec![None]
    };

    let mut totals = Vec::<Result>::new();

    let mut pb = if first {
        Some(Bar::new(me_options.len() * el_options.len()))
    } else {
        None
    };

    for me_data in &me_options {
        for el_data in &el_options {
            if let Some(pb) = &mut pb {
                pb.update(1);
            }

            let mut new_current = current.clone();

            if let &Some((me_to_id, me_length)) = me_data {
                if let Some((el_to_id, _)) = el_data {
                    if el_to_id == &me_to_id {
                        continue;
                    }
                }
                new_current.move_time_left_me = me_length;
                new_current.claimd_rooms.insert(me_to_id);
                new_current.room_me = me_to_id;
            }
            if let &Some((el_to_id, el_length)) = el_data {
                new_current.move_time_left_el = el_length;
                new_current.claimd_rooms.insert(el_to_id);
                new_current.room_el = el_to_id;
            }

            let released = new_current.update(rooms);

            let result = Result { total: released };

            let result = result + find_best(rooms, best_paths, new_current, saved, false);

            totals.push(result);
        }
    }
    let mut new_current = current.clone();

    assert!(!(new_current.move_time_left_el != 0 && new_current.move_time_left_me != 0));

    let mut released = 0;
    if new_current.move_time_left_el != 0 || new_current.move_time_left_me != 0 {
        released += new_current.update_non_null(rooms);
    }
    let stop_moving_total = Result {
        total: released + new_current.total_flow_rate * new_current.time_left,
    };
    totals.push(stop_moving_total);
    let best = totals.into_iter().max().unwrap();
    saved.insert(current, best.clone());
    best
}

fn find_shortest_path(rooms: &HashMap<RoomID, Room>, from: RoomID, to: RoomID) -> u64 {
    if from == to {
        return 0;
    }
    let mut visited = HashSet::new();
    visited.insert(from);
    let mut to_search = VecDeque::with_capacity(1);
    to_search.push_back((from, 0));

    while !to_search.is_empty() {
        let (current_id, current_l) = to_search.pop_front().unwrap();
        let current = rooms.get(&current_id).unwrap();

        for leads_to in &current.leads_to {
            if leads_to == &to {
                return current_l + 1;
            }
            if !visited.contains(&leads_to) {
                visited.insert(*leads_to);
                to_search.push_back((*leads_to, current_l + 1))
            }
        }
    }

    panic!("Didn't find path from {:?} to {:?}", from, to);
}

fn main() {
    // let input = include_str!("./example.txt");
    let input = include_str!("./input.txt");
    // Old (correct 2052):
    // me: [AA, UW, TG, KS, FG, AP, WY]
    // el: [AA, TQ, EG, KR, EK, VW, FX]
    //
    // New (wrong 2033):
    // me: [AA, FG, KS, TG, UW, MS, HT]
    // el: [AA, TQ, EG, KR, EK, VW, FX]

    let mut rooms = HashMap::new();

    for l in input.lines() {
        let l = l.strip_prefix("Valve ").unwrap();
        let (name, rest) = l.split_once(" has flow rate=").unwrap();
        let (flow_rate, leads_to) = match rest.split_once("; tunnels lead to valves ") {
            Some(result) => result,
            None => rest.split_once("; tunnel leads to valve ").unwrap(),
        };

        let name = name_to_room_id(name);
        let flow_rate = flow_rate.parse().unwrap();
        let leads_to = leads_to.split(", ").map(|id| name_to_room_id(id)).collect();

        rooms.insert(
            name,
            Room {
                flow_rate,
                leads_to,
            },
        );
    }
    let start = name_to_room_id("AA");

    let mut best_paths = HashMap::new();

    for from in rooms.keys() {
        let mut best_paths_from = HashMap::new();
        for to in rooms.keys() {
            if rooms.get(to).unwrap().flow_rate == 0 {
                continue;
            }
            let l = find_shortest_path(&rooms, *from, *to);
            best_paths_from.insert(*to, l);
        }
        best_paths.insert(*from, best_paths_from);
    }

    let current = Current {
        room_me: start,
        room_el: start,
        move_time_left_me: 0,
        move_time_left_el: 0,
        time_left: 26,
        total_flow_rate: 0,
        claimd_rooms: BTreeSet::new(),
    };

    let mut saved = HashMap::new();

    let best = find_best(&rooms, &best_paths, current, &mut saved, true);
    println!("{:?}", best);
}
