use std::{cmp::max, collections::BTreeSet};

use advent_of_code_2021::*;

aoc!(Day);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pt {
    x: i64,
    y: i64,
    z: i64,
}

impl std::ops::Add for Pt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Pt {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Pt {
    fn mdist(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn rot(&self, dir: usize) -> Pt {
        let rx = dir % 4;
        let ry = (dir / 4) % 4;
        let rz = (dir / 16) % 4;

        let mut ret = self.clone();
        for _ in 0..rx {
            ret = ret.rot_x();
        }
        for _ in 0..ry {
            ret = ret.rot_y();
        }
        for _ in 0..rz {
            ret = ret.rot_z();
        }
        ret
    }

    fn rot_x(&self) -> Pt {
        Pt {
            x: self.x,
            y: -self.z,
            z: self.y,
        }
    }

    fn rot_y(&self) -> Pt {
        Pt {
            x: -self.z,
            y: self.y,
            z: self.x,
        }
    }

    fn rot_z(&self) -> Pt {
        Pt {
            x: -self.y,
            y: self.x,
            z: self.z,
        }
    }
}

impl AOC for Day {
    type Input = Vec<Vec<Pt>>;

    type Output = i64;

    fn input() -> Result<Self::Input> {
        let c = get_contents()?;
        let mut it = c.lines();

        let mut ret = vec![];

        loop {
            let hdr = it.next();
            if hdr.is_none() {
                break;
            }

            eprintln!("{}", hdr.unwrap());

            let mut scanner = vec![];

            loop {
                let line = it.next();
                if line.is_none() || line.unwrap().is_empty() {
                    break;
                }
                let mut jt = line.unwrap().split(",");
                let x = jt.next().unwrap().parse::<i64>().unwrap();
                let y = jt.next().unwrap().parse::<i64>().unwrap();
                let z = jt.next().unwrap().parse::<i64>().unwrap();
                scanner.push(Pt { x, y, z });
            }

            ret.push(scanner);
        }

        Ok(ret)
    }

    fn part_one(input: &Self::Input) -> Self::Output {
        assemble(input).0.len() as i64
    }

    fn part_two(input: &Self::Input) -> Self::Output {
        let bcns = dbg!(assemble(input).1);
        let mut ret = 0;

        for i in 0..bcns.len() {
            for j in i + 1..bcns.len() {
                ret = max(ret, (bcns[i] - bcns[j]).mdist());
            }
        }

        ret
    }
}

fn assemble(input: &Vec<Vec<Pt>>) -> (Vec<Pt>, Vec<Pt>) {
    let mut done = vec![false; input.len()];
    done[0] = true;

    let mut ret = input[0].clone();
    let mut bcns = vec![];
    bcns.push(Pt { x: 0, y: 0, z: 0 });

    for ii in 1..input.len() {
        let mut ok = false;

        'outer: for i in 0..input.len() {
            if done[i] {
                continue;
            }

            for dir in 0..64 {
                let (pts, ofs) = merge(
                    &ret,
                    &input[i].iter().map(|p| p.rot(dir)).collect::<Vec<_>>(),
                );
                let overlaps = ret.len() + input[i].len() - pts.len();

                // eprintln!(
                //     "{:?}",
                //     (i, dir, ret.len(), input[i].len(), pts.len(), overlaps)
                // );

                if overlaps >= 12 {
                    eprintln!("merged {} by dir {}, new size = {}", i, dir, pts.len());
                    ret = pts;
                    done[i] = true;
                    bcns.push(ofs);
                    ok = true;

                    break 'outer;
                }
            }
        }

        assert!(ok, "{} cannot merged", ii);
    }

    (ret.into_iter().collect(), bcns)
}

fn merge(a: &Vec<Pt>, b: &Vec<Pt>) -> (Vec<Pt>, Pt) {
    let mut mts = vec![];

    let aa = a.iter().cloned().collect::<BTreeSet<_>>();

    for j in 0..a.len() {
        for i in 0..b.len() {
            let d = a[j] - b[i];

            let mut cur = 0;

            for j in 0..b.len() {
                if aa.contains(&(b[j] + d)) {
                    cur += 1;
                }
            }

            mts.push((cur, (i, j)));
        }
    }

    let (i, j) = mts.iter().max_by_key(|r| r.0).unwrap().1;
    let d = a[j] - b[i];

    let mut ret = vec![];

    for i in 0..b.len() {
        ret.push(b[i] + d);
    }

    (
        a.iter()
            .cloned()
            .chain(b.iter().map(|p| *p + d))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect(),
        d,
    )
}
