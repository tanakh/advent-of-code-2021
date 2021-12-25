use std::cmp::Reverse;

use advent_of_code_2021::*;

aoc!(Day);

impl AOC for Day {
    type Input = Vec<Vec<i64>>;

    type Output = i64;

    fn input() -> Result<Self::Input> {
        let c = get_contents()?;
        let ret = c
            .lines()
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as i64).collect())
            .collect();
        Ok(ret)
    }

    fn part_one(input: &Self::Input) -> Self::Output {
        let h = input.len();
        let w = input[0].len();

        let mut ret = 0;

        let vect = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        for y in 0..h {
            for x in 0..w {
                let mut ok = true;

                for (dx, dy) in vect {
                    let cx = x as isize + dx;
                    let cy = y as isize + dy;

                    let h2 = if cx >= 0 && cx < w as isize && cy >= 0 && cy < h as isize {
                        input[cy as usize][cx as usize]
                    } else {
                        10
                    };

                    if h2 <= input[y][x] {
                        ok = false;
                    }
                }

                if ok {
                    ret += input[y][x] + 1;
                }
            }
        }

        ret
    }

    fn part_two(input: &Self::Input) -> Self::Output {
        let mut bd = input.clone();

        let h = input.len();
        let w = input[0].len();

        let mut sizes = vec![];

        for y in 0..h {
            for x in 0..w {
                if bd[y][x] == 9 {
                    continue;
                }

                let size = paint(&mut bd, x, y);
                sizes.push(size as i64);
            }
        }

        sizes.sort_by_key(|s| Reverse(*s));

        sizes[0..3].iter().product::<i64>()
    }
}

fn paint(bd: &mut Vec<Vec<i64>>, x: usize, y: usize) -> usize {
    if bd[y][x] == 9 {
        return 0;
    }

    bd[y][x] = 9;

    let h = bd.len();
    let w = bd[0].len();

    let mut ret = 1;

    if x + 1 < w {
        ret += paint(bd, x + 1, y);
    }
    if x >= 1 {
        ret += paint(bd, x - 1, y);
    }
    if y + 1 < h {
        ret += paint(bd, x, y + 1);
    }
    if y >= 1 {
        ret += paint(bd, x, y - 1);
    }

    ret
}
