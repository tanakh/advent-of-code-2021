use std::cmp::{max, min};

use advent_of_code_2021::*;

aoc!(Day);

type Pt = (isize, isize);

impl AOC for Day {
    type Input = Vec<(Pt, Pt)>;

    type Output = usize;

    fn input() -> Result<Self::Input> {
        let re =
            regex::Regex::new(r"^(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)$").unwrap();

        let con = get_contents()?;
        let ret = con
            .lines()
            .map(|line| {
                let caps = re.captures(line).unwrap();
                (
                    (caps["x1"].parse().unwrap(), caps["y1"].parse().unwrap()),
                    (caps["x2"].parse().unwrap(), caps["y2"].parse().unwrap()),
                )
            })
            .collect();
        Ok(ret)
    }

    fn part_one(input: &Self::Input) -> Self::Output {
        solve(input, false)
    }

    fn part_two(input: &Self::Input) -> Self::Output {
        solve(input, true)
    }
}

fn solve(input: &Vec<(Pt, Pt)>, part_two: bool) -> usize {
    let mut bd = vec![[0; 1000]; 1000];

    for (p1, p2) in input {
        if p1.0 == p2.0 {
            for y in min(p1.1, p2.1)..=max(p1.1, p2.1) {
                bd[y as usize][p1.0 as usize] += 1;
            }
        } else if p1.1 == p2.1 {
            for x in min(p1.0, p2.0)..=max(p1.0, p2.0) {
                bd[p1.1 as usize][x as usize] += 1;
            }
        } else {
            if part_two {
                assert_eq!((p1.0 - p2.0).abs(), (p1.1 - p2.1).abs());

                let len = (p1.0 - p2.0).abs();
                let dx = (p2.0 - p1.0).signum();
                let dy = (p2.1 - p1.1).signum();

                for i in 0..=len {
                    let cx = p1.0 + i * dx;
                    let cy = p1.1 + i * dy;
                    bd[cy as usize][cx as usize] += 1;
                }
            }
        }
    }

    bd.iter().flatten().filter(|&&x| x > 1).count()
}
