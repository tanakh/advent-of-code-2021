use advent_of_code_2021::*;

aoc!(Day);

struct Input {
    dict: Vec<bool>,
    bd: Vec<Vec<bool>>,
}

impl AOC for Day {
    type Input = Input;

    type Output = i64;

    fn input() -> Result<Self::Input> {
        let c = get_contents()?;
        let mut it = c.lines();

        let dict = it.next().unwrap().chars().map(|c| c == '#').collect();
        assert_eq!(it.next().unwrap(), "");

        let bd = it.map(|l| l.chars().map(|c| c == '#').collect()).collect();

        let ret = Input { dict, bd };

        Ok(ret)
    }

    fn part_one(input: &Self::Input) -> Self::Output {
        let bd = enhance(&input.bd, &input.dict, 2);
        bd.iter()
            .map(|r| r.iter().filter(|&&b| b).count())
            .sum::<usize>() as i64
    }

    fn part_two(input: &Self::Input) -> Self::Output {
        let bd = enhance(&input.bd, &input.dict, 50);
        bd.iter()
            .map(|r| r.iter().filter(|&&b| b).count())
            .sum::<usize>() as i64
    }
}

fn enhance(init: &Vec<Vec<bool>>, dict: &[bool], turns: usize) -> Vec<Vec<bool>> {
    let mergin = turns + 3;

    let h = init.len();
    let w = init[0].len();
    let mut bd = vec![vec![false; w + mergin * 2]; h + mergin * 2];

    for y in 0..h {
        for x in 0..w {
            bd[y + mergin][x + mergin] = init[y][x];
        }
    }

    for turn in 0..turns {
        let bg = if dict[0] { turn % 2 == 0 } else { false };

        let mut next = vec![vec![bg; w + mergin * 2]; h + mergin * 2];

        for y in 1..bd.len() - 1 {
            for x in 1..bd[y].len() - 1 {
                let mut b = 0;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        b = b * 2
                            + bd[(y as isize + dy) as usize][(x as isize + dx) as usize] as usize;
                    }
                }

                next[y][x] = dict[b];
            }
        }

        bd = next;

        // for row in &bd {
        //     for cell in row {
        //         print!("{}", if *cell { '#' } else { '.' });
        //     }
        //     println!();
        // }
        // println!();
    }

    bd
}
