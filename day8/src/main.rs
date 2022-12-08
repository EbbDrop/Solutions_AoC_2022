fn main() {
    let lines = include_str!("./input.txt").lines();
    // let lines = include_str!("./example.txt").lines();

    let mut forrest: Vec<Vec<i8>> = Vec::new();

    for line in lines {
        let forrest_line = line.bytes().map(|c| (c - '0' as u8) as i8).collect();
        forrest.push(forrest_line);
    }

    let width = forrest[0].len() as i32;
    let hight = forrest.len() as i32;

    let mut scores = Vec::new();

    for y in 0..hight {
        for x in 0..width {
            let this_hight = forrest[y as usize][x as usize];

            let right = ((x + 1)..width)
                .enumerate()
                .map(|(i, x)| (i as i32, forrest[y as usize][x as usize]))
                .find(|(_, h)| h >= &this_hight)
                .map(|(i, _)| i + 1)
                .unwrap_or(width - x - 1);

            let left = (0..=(x - 1))
                .rev()
                .enumerate()
                .map(|(i, x)| (i as i32, forrest[y as usize][x as usize]))
                .find(|(_, h)| h >= &this_hight)
                .map(|(i, _)| i + 1)
                .unwrap_or(x);

            let down = ((y + 1)..hight)
                .enumerate()
                .map(|(i, y)| (i as i32, forrest[y as usize][x as usize]))
                .find(|(_, h)| h >= &this_hight)
                .map(|(i, _)| i + 1)
                .unwrap_or(hight - y - 1);

            let up = (0..=(y - 1))
                .rev()
                .enumerate()
                .map(|(i, y)| (i as i32, forrest[y as usize][x as usize]))
                .find(|(_, h)| h >= &this_hight)
                .map(|(i, _)| i + 1)
                .unwrap_or(y);

            let score = right * left * down * up;
            scores.push(score);
        }
    }
    dbg!(scores.into_iter().max());
}
