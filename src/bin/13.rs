use std::cmp::max;

use advent_of_code_2021::*;

aoc!(Day);

#[derive(Debug)]
struct Input {
    pts: Vec<Pt>,
    folds: Vec<(Dir, usize)>,
}

type Pt = (usize, usize);

#[derive(Debug, Clone, Copy)]
enum Dir {
    X,
    Y,
}

impl AOC for Day {
    type Input = Input;

    type Output = usize;

    fn input() -> Result<Self::Input> {
        let c = get_contents()?;

        let mut lines = c.lines();

        let mut pts = vec![];

        loop {
            let line = lines.next().unwrap();
            if line == "" {
                break;
            }

            let mut it = line.split(",");
            let x = it.next().unwrap().parse::<usize>()?;
            let y = it.next().unwrap().parse::<usize>()?;

            pts.push((x, y));
        }

        let mut folds = vec![];

        let re = regex::Regex::new(r"^fold along (?P<dir>\w+)=(?P<pos>\d+)$")?;

        for line in lines {
            let caps = re.captures(line).unwrap();
            let dir = match &caps["dir"] {
                "x" => Dir::X,
                "y" => Dir::Y,
                _ => panic!("invalid dir"),
            };
            let pos = caps["pos"].parse::<usize>()?;

            folds.push((dir, pos));
        }

        Ok(Input { pts, folds })
    }

    fn part_one(input: &Self::Input) -> Self::Output {
        let mut bd = build(&input);

        fold(input.folds[0].0, input.folds[0].1, &mut bd);

        bd.iter()
            .map(|row| row.iter().filter(|c| **c == '#').count())
            .sum()
    }

    fn part_two(input: &Self::Input) -> Self::Output {
        let mut bd = build(&input);

        for &(dir, pos) in input.folds.iter() {
            fold(dir, pos, &mut bd);
        }

        dump(&bd);

        0
    }
}

fn build(input: &Input) -> Vec<Vec<char>> {
    let mut w = 0;
    let mut h = 0;

    for &(x, y) in input.pts.iter() {
        w = max(w, x + 1);
        h = max(h, y + 1);
    }

    let mut bd = vec![vec!['.'; w]; h];

    for &(x, y) in input.pts.iter() {
        bd[y][x] = '#';
    }

    bd
}

fn dump(bd: &Vec<Vec<char>>) {
    for row in bd.iter() {
        for &c in row.iter() {
            print!("{}", if c == '.' { ' ' } else { c });
        }
        println!();
    }
}

fn fold(dir: Dir, pos: usize, bd: &mut Vec<Vec<char>>) {
    let h = bd.len();
    let w = bd[0].len();

    match dir {
        Dir::X => {
            for y in 0..h {
                assert_eq!(bd[y][pos], '.');
            }

            for y in 0..h {
                for x in pos + 1..w {
                    if bd[y][x] != '.' {
                        assert!(pos * 2 >= x);
                        let rx = pos * 2 - x;
                        bd[y][rx] = '#';
                    }
                }
            }

            for y in 0..h {
                bd[y].resize(pos, ',');
            }
        }
        Dir::Y => {
            for x in 0..w {
                assert_eq!(bd[pos][x], '.');
            }

            for y in pos + 1..h {
                for x in 0..w {
                    if bd[y][x] != '.' {
                        assert!(pos * 2 >= y);
                        let ry = pos * 2 - y;
                        bd[ry][x] = '#';
                    }
                }
            }

            bd.resize(pos, vec![]);
        }
    }
}
