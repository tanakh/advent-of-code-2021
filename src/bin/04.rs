use std::cmp::Reverse;

use advent_of_code_2021::*;

aoc!(Day);

struct Input {
    call: Vec<usize>,
    boards: Vec<Vec<Vec<usize>>>,
}

impl AOC for Day {
    type Input = Input;

    type Output = usize;

    fn input() -> Result<Self::Input> {
        let call: Vec<usize> = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s)?;
            s.trim().split(',').map(|w| w.parse().unwrap()).collect()
        };

        let boards: Vec<Vec<Vec<usize>>> = {
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s)?;
            s.split_ascii_whitespace()
                .map(|w| w.parse().unwrap())
                .collect::<Vec<_>>()
                .chunks(25)
                .map(|b| b.chunks(5).map(|s| s.to_vec()).collect())
                .collect()
        };

        for board in &boards {
            assert_eq!(board.len(), 5);
            for row in board {
                assert_eq!(row.len(), 5);
            }
        }

        Ok(Input { call, boards })
    }

    fn part_one(input: &Self::Input) -> Self::Output {
        let mut cand = solve(input);

        cand.sort();

        assert!(cand.len() == 1 || cand[0].1 != cand[1].1);

        cand[0].1 * cand[0].2
    }

    fn part_two(input: &Self::Input) -> Self::Output {
        let mut cand = solve(input);

        cand.sort_by_key(|&v| Reverse(v));

        assert!(cand.len() == 1 || cand[0].1 != cand[1].1);

        cand[0].1 * cand[0].2
    }
}

fn solve(input: &Input) -> Vec<(usize, usize, usize)> {
    let mut cand = vec![];

    for board in &input.boards {
        let mut done = [[false; 5]; 5];

        for (ix, &c) in input.call.iter().enumerate() {
            for i in 0..5 {
                for j in 0..5 {
                    if board[i][j] == c {
                        done[i][j] = true;
                    }
                }
            }

            let mut ok = false;

            'outer: for i in 0..5 {
                for j in 0..5 {
                    if !done[i][j] {
                        continue 'outer;
                    }
                }
                ok = true;
                break;
            }

            'outer2: for j in 0..5 {
                for i in 0..5 {
                    if !done[i][j] {
                        continue 'outer2;
                    }
                }
                ok = true;
                break;
            }

            if ok {
                let mut score = 0;

                for i in 0..5 {
                    for j in 0..5 {
                        if !done[i][j] {
                            score += board[i][j];
                        }
                    }
                }

                cand.push((ix, c, score));
                break;
            }
        }
    }

    assert!(!cand.is_empty());

    cand
}
