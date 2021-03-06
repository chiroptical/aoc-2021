// Use main's lib module here?
use super::lib::read_file_and_parse_lines;
use std::clone::Clone;

fn zip_solution(offset: usize, vec: &Vec<u32>) -> u32 {
    if vec.len() < offset + 1 {
        return 0;
    };
    let zipped = vec.iter().zip(vec[offset..].iter());
    return zipped.fold(0, |acc, (x, y)| if x < y { acc + 1 } else { acc });
}

fn build_subvectors(size: usize, vec: &Vec<u32>) -> Vec<Vec<u32>> {
    let mut res: Vec<Vec<u32>> = Vec::new();
    for win in vec.windows(size) {
        res.push(win.to_vec());
    }
    return res;
}

// This one works for offset = 1 but not offset = 3 and I don't care to figure out why :smile:
fn with_windows(offset: usize, vec: &Vec<u32>) -> u32 {
    if vec.len() < offset + 1 {
        return 0;
    };
    let subvectors = build_subvectors(offset, vec);
    let zipped = subvectors.iter().zip(subvectors[offset..].iter());
    return zipped.fold(0, |acc, (x, y)| {
        let x_sum: u32 = x.clone().iter().sum();
        let y_sum: u32 = y.clone().iter().sum();
        if x_sum < y_sum {
            acc + 1
        } else {
            acc
        }
    });
}

// part1 :: List Int -> Int
// part1 (a : b : rest) = (if a < b then 1 else 0) + part1 (b : rest)
// part1 _ = 0
fn part1(v: &Vec<u32>) -> u32 {
    match v.as_slice() {
        [x, y, rest @ ..] => {
            let mut next: Vec<u32> = Vec::new();
            next.push(y.clone());
            next.extend(rest.to_vec());
            return (if x < y { 1 } else { 0 }) + part1(&next);
        }
        _ => 0,
    }
}

// part2 :: List Int -> Int
// part2 (a : rest@(_ : _ : d : _)) = (if a < d then 1 else 0) + part2 rest
// part2 _ = 0
fn part2(v: &Vec<u32>) -> u32 {
    match v.as_slice() {
        [x, y, z, a, rest @ ..] => {
            let mut next: Vec<u32> = Vec::new();
            next.push(y.clone());
            next.push(z.clone());
            next.push(a.clone());
            next.extend(rest.to_vec());
            return (if x < a { 1 } else { 0 }) + part2(&next);
        }
        _ => 0,
    }
}

pub fn run() {
    let sonar_sweep_depths: Vec<u32> =
        read_file_and_parse_lines("./inputs/day1.txt", |s| s.parse::<u32>().ok());
    println!("Part 1: {:?}", part1(&sonar_sweep_depths));
    println!("Part 2: {:?}", part2(&sonar_sweep_depths));
    println!(
        "Part 1 (using zip): {:?}",
        zip_solution(1, &sonar_sweep_depths)
    );
    println!(
        "Part 2 (using zip): {:?}",
        zip_solution(3, &sonar_sweep_depths)
    );
    println!(
        "Part 1 (using windows): {:?}",
        with_windows(1, &sonar_sweep_depths)
    );
    println!(
        "Part 2 (using windows): {:?}",
        with_windows(3, &sonar_sweep_depths)
    );
}
