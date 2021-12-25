use std::{
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet},
};

use advent_of_code_2021::*;

aoc!(Day);

#[derive(Debug)]
struct Step {
    on: bool,
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

impl AOC for Day {
    type Input = Vec<Step>;

    type Output = usize;

    fn input() -> Result<Self::Input> {
        let re = regex::Regex::new(
            r"^(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)$",
        )
        .unwrap();

        let mut ret = vec![];

        for line in get_contents()?.lines() {
            let caps = re.captures(line).unwrap();

            let on = match &caps[1] {
                "on" => true,
                "off" => false,
                _ => unreachable!(),
            };

            let sx = caps[2].parse().unwrap();
            let ex = caps[3].parse().unwrap();
            let sy = caps[4].parse().unwrap();
            let ey = caps[5].parse().unwrap();
            let sz = caps[6].parse().unwrap();
            let ez = caps[7].parse().unwrap();

            ret.push(Step {
                on,
                x: (sx, ex),
                y: (sy, ey),
                z: (sz, ez),
            });
        }

        Ok(ret)
    }

    fn part_one(input: &Self::Input) -> Self::Output {
        let mut ss = BTreeSet::new();

        let lo = -50;
        let hi = 50;

        for step in input {
            for x in max(lo, step.x.0)..=min(hi, step.x.1) {
                for y in max(lo, step.y.0)..=min(hi, step.y.1) {
                    for z in max(lo, step.z.0)..=min(hi, step.z.1) {
                        if step.on {
                            ss.insert((x, y, z));
                        } else {
                            ss.remove(&(x, y, z));
                        }
                    }
                }
            }
        }

        ss.len()
    }

    fn part_two(input: &Self::Input) -> Self::Output {
        let mut xs = BTreeSet::new();
        let mut ys = BTreeSet::new();
        let mut zs = BTreeSet::new();

        for step in input {
            xs.insert(step.x.0);
            xs.insert(step.x.0 + 1);
            xs.insert(step.x.1);
            xs.insert(step.x.1 + 1);
            ys.insert(step.y.0);
            ys.insert(step.y.0 + 1);
            ys.insert(step.y.1);
            ys.insert(step.y.1 + 1);
            zs.insert(step.z.0);
            zs.insert(step.z.0 + 1);
            zs.insert(step.z.1);
            zs.insert(step.z.1 + 1);
        }

        let xs = xs
            .into_iter()
            .enumerate()
            .map(|(a, b)| (b, a))
            .collect::<BTreeMap<_, _>>();
        let ys = ys
            .into_iter()
            .enumerate()
            .map(|(a, b)| (b, a))
            .collect::<BTreeMap<_, _>>();
        let zs = zs
            .into_iter()
            .enumerate()
            .map(|(a, b)| (b, a))
            .collect::<BTreeMap<_, _>>();

        let mut ss = vec![vec![vec![false; xs.len() + 1]; ys.len() + 1]; zs.len() + 1];

        for step in input.iter() {
            for x in xs[&step.x.0]..=xs[&step.x.1] {
                for y in ys[&step.y.0]..=ys[&step.y.1] {
                    for z in zs[&step.z.0]..=zs[&step.z.1] {
                        if step.on {
                            ss[z][y][x] = true;
                        } else {
                            ss[z][y][x] = false;
                        }
                    }
                }
            }
        }

        let xs = xs.into_iter().map(|(v, _)| v).collect::<Vec<_>>();
        let ys = ys.into_iter().map(|(v, _)| v).collect::<Vec<_>>();
        let zs = zs.into_iter().map(|(v, _)| v).collect::<Vec<_>>();

        let mut ret = 0;

        for z in 0..ss.len() - 1 {
            for y in 0..ss[z].len() - 1 {
                for x in 0..ss[z][y].len() - 1 {
                    if ss[z][y][x] {
                        let w = xs[x + 1] - xs[x];
                        let h = ys[y + 1] - ys[y];
                        let d = zs[z + 1] - zs[z];

                        ret += w * h * d;
                    }
                }
            }
        }

        ret as usize
    }
}
