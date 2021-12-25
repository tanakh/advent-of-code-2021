use std::{
    cmp::{min, Reverse},
    collections::BinaryHeap,
};

use advent_of_code_2021::*;

aoc!(Day);

impl AOC for Day {
    type Input = Vec<Vec<i64>>;

    type Output = i64;

    fn input() -> Result<Self::Input> {
        let c = get_contents()?;
        Ok(c.lines()
            .map(|line| {
                line.chars()
                    .map(|s| s.to_digit(10).unwrap() as i64)
                    .collect()
            })
            .collect())
    }

    fn part_one(input: &Self::Input) -> Self::Output {
        solve(0, 0, &input) - input[0][0]
    }

    fn part_two(input: &Self::Input) -> Self::Output {
        let h = input.len();
        let w = input[0].len();

        let mut bd = vec![vec![0; w * 5]; h * 5];

        for y in 0..bd.len() {
            for x in 0..bd[0].len() {
                let lx = x % w;
                let ly = y % h;

                let nx = x / w;
                let ny = y / h;

                bd[y][x] = {
                    let t = input[ly][lx] + nx as i64 + ny as i64;
                    (t - 1) % 9 + 1
                };
            }
        }

        let h = bd.len();
        let w = bd[0].len();

        let mut q = BinaryHeap::new();
        q.push((Reverse(0), 0, 0));
        let mut done = vec![vec![false; w]; h];
        done[0][0] = true;

        while let Some((Reverse(d), x, y)) = q.pop() {
            if (x, y) == (w - 1, h - 1) {
                return d;
            }

            for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let nx = x as i64 + dx;
                let ny = y as i64 + dy;

                if nx < 0 || ny < 0 || nx >= w as i64 || ny >= h as i64 {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;

                if done[ny][nx] {
                    continue;
                }

                let nd = d + bd[ny][nx];
                q.push((Reverse(nd), nx, ny));
                done[ny][nx] = true;
            }
        }

        unreachable!()
    }
}

#[memoise::memoise(x, y)]
fn solve(x: usize, y: usize, bd: &Vec<Vec<i64>>) -> i64 {
    let h = bd.len();
    let w = bd[0].len();

    let cur = bd[y][x];

    if (x, y) == (w - 1, h - 1) {
        return cur;
    }

    let mut ret = i64::MAX;

    if x + 1 < w {
        ret = min(ret, solve(x + 1, y, bd) + cur);
    }
    if y + 1 < h {
        ret = min(ret, solve(x, y + 1, bd) + cur);
    }

    ret
}
