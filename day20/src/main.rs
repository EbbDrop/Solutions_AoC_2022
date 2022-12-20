use itertools::Either;

fn get_range_iter(start: usize, end: usize) -> impl Iterator<Item = usize> {
    if start < end {
        Either::Left(start..=end)
    } else {
        Either::Right((end..=start).rev())
    }
}

fn main() {
    // let input = include_str!("./example.txt");
    let input = include_str!("./input.txt");

    let mut nums = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap() * 811589153)
        .collect::<Vec<_>>();

    let mut indexes = (0..nums.len()).collect::<Vec<_>>();

    for dbggg in 0..10 {
        dbg!(dbggg);
        for i in 0..indexes.len() {
            let index = indexes[i];
            let mut new_index =
                (index as i64 + nums[index]).rem_euclid(indexes.len() as i64 - 1) as usize;
            if new_index == 0 {
                new_index = indexes.len() - 1;
            }

            let old_num = nums.remove(index);
            nums.insert(new_index, old_num);

            let dir = -(new_index as i64 - index as i64).signum();

            // println!(
            //     "Moving: {} at {} to {} so dir is {}",
            //     old_num, index, new_index, dir
            // );

            for j in 0..indexes.len() {
                if index.min(new_index) <= indexes[j] && indexes[j] <= index.max(new_index) {
                    indexes[j] = indexes[j].saturating_add_signed(dir as isize) as usize;
                }
            }
            indexes[i] = new_index;
        }
    }

    let mut sum = 0;
    let index_zero = nums
        .iter()
        .enumerate()
        .find_map(|(i, v)| if *v == 0 { Some(i) } else { None })
        .unwrap();
    for i in &[1000i64, 2000, 3000] {
        sum += dbg!(nums[(i + index_zero as i64).rem_euclid(indexes.len() as i64) as usize]);
    }
    dbg!(sum);
}
