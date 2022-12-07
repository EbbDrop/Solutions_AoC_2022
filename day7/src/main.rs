use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
struct File {
    size: u64,
}

#[derive(Debug, Clone)]
enum Object {
    File(File),
    Dir(Dir),
}

#[derive(Debug, Clone, Default)]
struct Dir {
    objects: HashMap<String, Rc<RefCell<Object>>>,
    chached_size: Option<u64>,
}
impl Dir {
    fn calc_size(&mut self) -> u64 {
        let mut size = 0;

        for object in self.objects.values() {
            size += match &mut *object.borrow_mut() {
                Object::File(f) => f.size,
                Object::Dir(d) => d.calc_size(),
            }
        }

        self.chached_size = Some(size);
        size
    }

    fn run_on_dirs(&self, f: &mut dyn FnMut(&Dir)) {
        f(self);
        for object in self.objects.values() {
            match &*object.borrow_mut() {
                Object::Dir(d) => d.run_on_dirs(f),
                _ => {}
            }
        }
    }
}

fn main() {
    // let mut lines = include_str!("./test.txt").lines().peekable();
    let mut lines = include_str!("./input.txt").lines().peekable();

    let system = Rc::new(RefCell::new(Object::Dir(Dir::default())));
    let mut curr = vec![system.clone()];

    while let Some(command) = lines.next() {
        match &command[..4] {
            "$ cd" => match &command[5..] {
                "/" => {
                    curr = vec![system.clone()];
                }
                ".." => {
                    curr.pop().unwrap();
                }
                name => {
                    let new_curr = if let Object::Dir(ref mut dir) =
                        &mut *(curr.last().unwrap()).borrow_mut()
                    {
                        dir.objects
                            .entry(name.to_owned())
                            .or_insert_with(|| Rc::new(RefCell::new(Object::Dir(Dir::default()))))
                            .clone()
                    } else {
                        panic!("Curr should always be a dir!")
                    };
                    curr.push(new_curr);
                }
            },
            "$ ls" => {
                while let Some(next_l) = lines.peek() {
                    if &next_l[..1] == "$" {
                        break;
                    }
                    if let Some(line) = lines.next() {
                        if line.starts_with("dir") {
                            continue;
                        }
                        let (size, name) = line.split_once(" ").unwrap();
                        let file = File {
                            size: size.parse().unwrap(),
                        };
                        if let Object::Dir(ref mut dir) = &mut *(curr.last().unwrap()).borrow_mut()
                        {
                            dir.objects
                                .entry(name.to_owned())
                                .or_insert_with(|| Rc::new(RefCell::new(Object::File(file))));
                        } else {
                            panic!("Curr should always be a dir!")
                        };
                    } else {
                        unreachable!();
                    }
                }
            }
            _ => panic!("Unknow command: {}", command),
        }
    }
    drop(curr);

    if let Object::Dir(ref mut dir) = &mut *system.borrow_mut() {
        dir.calc_size();
        let total = dir.chached_size.unwrap();

        dbg!(&total);
        let used = 70000000 - total as i64;
        dbg!(&used);
        let needed = 30000000 - used;
        dbg!(&needed);

        let mut ops = Vec::new();
        dir.run_on_dirs(&mut |d| {
            if d.chached_size.unwrap() as i64 >= needed {
                ops.push(d.chached_size.unwrap());
            }
        });
        dbg!(ops.iter().min());
    };
}
