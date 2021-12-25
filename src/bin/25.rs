use advent_of_code_2021::*;

aoc!(Day);

impl AOC for Day {
    type Input = Vec<Vec<char>>;

    type Output = i64;

    fn input() -> Result<Self::Input> {
        let c = get_contents()?;
        Ok(c.lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>())
    }

    fn part_one(input: &Self::Input) -> Self::Output {
        let h = input.len();
        let w = input[0].len();

        let mut bd = input.clone();
        for step in 1.. {
            let mut changed = false;

            let mut next = bd.clone();

            for y in 0..h {
                for x in 0..w {
                    let nx = (x + 1) % w;

                    if bd[y][x] == '>' {
                        if bd[y][nx] == '.' {
                            next[y][x] = '.';
                            next[y][nx] = '>';
                            changed = true;
                        } else {
                            next[y][x] = '>';
                        }
                    }
                }
            }

            bd = next;

            let mut next = bd.clone();

            for y in 0..h {
                for x in 0..w {
                    let ny = (y + 1) % h;

                    if bd[y][x] == 'v' {
                        if bd[ny][x] == '.' {
                            next[y][x] = '.';
                            next[ny][x] = 'v';
                            changed = true;
                        } else {
                            next[y][x] = 'v';
                        }
                    }
                }
            }

            if !changed {
                return step;
            }

            bd = next;
        }
        unreachable!()
    }

    fn part_two(_input: &Self::Input) -> Self::Output {
        panic!("Nothing to do!")
    }
}
