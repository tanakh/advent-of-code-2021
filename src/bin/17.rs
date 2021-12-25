use std::cmp::max;

use advent_of_code_2021::*;
use num_complex::Complex;

aoc!(Day);

type Pt = Complex<i64>;

impl AOC for Day {
    type Input = (Pt, Pt);

    type Output = i64;

    fn input() -> Result<Self::Input> {
        // Ok((Complex::new(20, -10), Complex::new(30, -5)))
        Ok((Complex::new(156, -110), Complex::new(202, -69)))
    }

    fn part_one(input: &Self::Input) -> Self::Output {
        let mut ret = 0;

        for dy in -1000..1000 {
            for dx in 0..1000 {
                if let Some(maxh) = check(Complex::new(dx, dy), input) {
                    ret = max(ret, maxh);
                }
            }
        }

        ret
    }

    fn part_two(input: &Self::Input) -> Self::Output {
        let mut ret = 0;

        for dy in -1000..1000 {
            for dx in 0..1000 {
                if check(Complex::new(dx, dy), input).is_some() {
                    ret += 1;
                }
            }
        }

        ret
    }
}

fn check(mut v: Pt, target: &(Pt, Pt)) -> Option<i64> {
    let mut cur = Complex::new(0, 0);
    let mut maxh = 0;

    while cur.im >= target.0.im {
        cur += v;
        v.im -= 1;
        if v.re > 0 {
            v.re -= 1;
        }

        maxh = max(maxh, cur.im);

        if target.0.re <= cur.re
            && cur.re <= target.1.re
            && target.0.im <= cur.im
            && cur.im <= target.1.im
        {
            return Some(maxh);
        }
    }

    None
}
