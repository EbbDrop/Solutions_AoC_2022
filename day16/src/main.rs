use std::collections::{HashMap, HashSet, VecDeque};

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

#[derive(Debug, Clone)]
struct Current {
    room_me: RoomID,
    room_el: RoomID,
    move_time_left_me: u64,
    move_time_left_el: u64,

    time_left: u64,
    total_flow_rate: u64,
    opend_rooms: HashSet<RoomID>,
    total: u64,

    path_me: Vec<RoomID>,
    path_el: Vec<RoomID>,
}

fn find_best(
    rooms: &HashMap<RoomID, Room>,
    best_paths: &HashMap<RoomID, HashMap<RoomID, u64>>,
    mut current: Current,
    best: &mut u64,
) {
    if current.time_left == 0 {
        if current.total > *best {
            *best = current.total;
            // println!(
            //     "me: {:?}\nel: {:?}\nscore: {}",
            //     current.path_me, current.path_el, best
            // );
            println!(
                "me: {:?} {:?}\nel: {:?} {:?}\nscore: {}, mtel: {}, mtme: {}, tl: {}",
                current.path_me,
                current.room_me,
                current.path_el,
                current.room_el,
                current.total,
                current.move_time_left_el,
                current.move_time_left_me,
                current.time_left,
            );
        }
        return;
    }
    debug_assert!(current.time_left > 0);

    if current.move_time_left_me == 0 && !current.path_me.contains(&current.room_me) {
        current.path_me.push(current.room_me);
        current.total_flow_rate += rooms.get(&current.room_me).unwrap().flow_rate;
    }
    if current.move_time_left_el == 0 && !current.path_el.contains(&current.room_el) {
        current.path_el.push(current.room_el);
        current.total_flow_rate += rooms.get(&current.room_el).unwrap().flow_rate;
    }

    // if current.path_me.starts_with(&[
    //     name_to_room_id("AA"),
    //     name_to_room_id("JJ"),
    //     name_to_room_id("BB"),
    // ]) && current.path_el.starts_with(&[
    //     name_to_room_id("AA"),
    //     name_to_room_id("DD"),
    //     name_to_room_id("HH"),
    // ]) {
    //     println!(
    //         "me: {:?} {:?}\nel: {:?} {:?}\nscore: {}, mtel: {}, mtme: {}, tl: {}",
    //         current.path_me,
    //         current.room_me,
    //         current.path_el,
    //         current.room_el,
    //         current.total,
    //         current.move_time_left_el,
    //         current.move_time_left_me,
    //         current.time_left,
    //     );
    // }

    if current.move_time_left_me == 0 && current.move_time_left_el == 0 {
        for (&me_to_id, &me_length) in best_paths.get(&current.room_me).unwrap().iter() {
            let me_length = me_length + 1;
            if current.opend_rooms.contains(&me_to_id) {
                continue;
            }
            let me_room = rooms.get(&me_to_id).unwrap();
            if me_room.flow_rate == 0 || current.time_left < me_length + 1 {
                continue;
            }
            for (&el_to_id, &el_length) in best_paths.get(&current.room_el).unwrap().iter() {
                if el_to_id == me_to_id {
                    continue;
                }
                let el_length = el_length + 1;
                if current.opend_rooms.contains(&el_to_id) {
                    continue;
                }
                let el_room = rooms.get(&el_to_id).unwrap();
                if el_room.flow_rate == 0 || current.time_left < el_length + 1 {
                    continue;
                }

                let mut new_current = current.clone();
                new_current.move_time_left_me = me_length;
                new_current.move_time_left_el = el_length;
                new_current.opend_rooms.insert(el_to_id);
                new_current.opend_rooms.insert(me_to_id);
                new_current.room_me = me_to_id;
                new_current.room_el = el_to_id;

                let used_time = if me_length < el_length {
                    me_length
                } else {
                    el_length
                };

                new_current.move_time_left_me -= used_time;
                new_current.move_time_left_el -= used_time;
                new_current.total += new_current.total_flow_rate * used_time;
                new_current.time_left -= used_time;

                find_best(rooms, best_paths, new_current, best);
            }
        }
        let mut new_current = current.clone();

        new_current.total += new_current.total_flow_rate * new_current.time_left;
        new_current.time_left = 0;

        find_best(rooms, best_paths, new_current, best);
    } else if current.move_time_left_me == 0 {
        for (&to_id, &length) in best_paths.get(&current.room_me).unwrap().iter() {
            let length = length + 1;
            if current.opend_rooms.contains(&to_id) {
                continue;
            }
            let room = rooms.get(&to_id).unwrap();
            if room.flow_rate == 0 || current.time_left < length + 1 {
                continue;
            }

            let mut new_current = current.clone();
            new_current.move_time_left_me = length;

            let used_time = if length > current.move_time_left_el {
                current.move_time_left_el
            } else {
                length
            };

            new_current.move_time_left_me -= used_time;
            new_current.move_time_left_el -= used_time;
            new_current.total += new_current.total_flow_rate * used_time;
            new_current.time_left -= used_time;
            new_current.room_me = to_id;
            new_current.opend_rooms.insert(to_id);

            find_best(rooms, best_paths, new_current, best);
        }
        let mut new_current = current.clone();

        new_current.total += new_current.total_flow_rate * current.move_time_left_el;
        new_current.time_left -= current.move_time_left_el;
        new_current.move_time_left_el = 0;

        find_best(rooms, best_paths, new_current, best);
    } else {
        for (&to_id, &length) in best_paths.get(&current.room_el).unwrap().iter() {
            let length = length + 1;
            if current.opend_rooms.contains(&to_id) {
                continue;
            }
            let room = rooms.get(&to_id).unwrap();
            if room.flow_rate == 0 || current.time_left < length + 1 {
                continue;
            }

            let mut new_current = current.clone();
            new_current.move_time_left_el = length;

            let used_time = if length > current.move_time_left_me {
                current.move_time_left_me
            } else {
                length
            };

            new_current.move_time_left_me -= used_time;
            new_current.move_time_left_el -= used_time;
            new_current.total += new_current.total_flow_rate * used_time;
            new_current.time_left -= used_time;
            new_current.room_el = to_id;
            new_current.opend_rooms.insert(to_id);

            find_best(rooms, best_paths, new_current, best);
        }
        let mut new_current = current.clone();

        new_current.total += new_current.total_flow_rate * current.move_time_left_me;
        new_current.time_left -= current.move_time_left_me;
        new_current.move_time_left_me = 0;

        find_best(rooms, best_paths, new_current, best);
    }
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
        opend_rooms: HashSet::new(),
        total: 0,

        path_me: Vec::new(),
        path_el: Vec::new(),
    };

    let mut best = 0;
    find_best(&rooms, &best_paths, current, &mut best);
    dbg!(best);
}
