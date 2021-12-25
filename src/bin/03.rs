use advent_of_code_2021::*;

aoc!(Day);

impl AOC for Day {
    type Input = Vec<Vec<i64>>;

    type Output = i64;

    fn input() -> Result<Self::Input> {
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s)?;

        let xs = s
            .lines()
            .map(|w| {
                w.chars()
                    .map(|c| c.to_digit(10).unwrap() as i64)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Ok(xs)
    }

    fn part_one(input: &Self::Input) -> Self::Output {
        let len = input[0].len();
        let mut gamma = 0;
        let mut eps = 0;

        for j in 0..len {
            let mut c0 = 0;
            let mut c1 = 0;
            for i in 0..input.len() {
                if input[i][j] == 0 {
                    c0 += 1;
                } else {
                    c1 += 1;
                }
            }

            assert!(c0 != c1);

            gamma = gamma * 2 + if c0 > c1 { 0 } else { 1 };
            eps = eps * 2 + if c0 > c1 { 1 } else { 0 };
        }

        gamma * eps
    }

    fn part_two(input: &Self::Input) -> Self::Output {
        let f = |sel| {
            let mut v = input.clone();

            for j in 0..v[0].len() {
                let mut c0 = 0;
                let mut c1 = 0;
                for i in 0..v.len() {
                    if v[i][j] == 0 {
                        c0 += 1;
                    } else {
                        c1 += 1;
                    }
                }

                let b = if c1 >= c0 { sel } else { sel ^ 1 };

                v = v.into_iter().filter(|r| r[j] == b).collect();

                assert!(v.len() > 0);

                if v.len() == 1 {
                    break;
                }
            }

            assert!(v.len() == 1, "{}", v.len());

            v[0].iter().fold(0, |a, b| a * 2 + b)
        };

        f(1) * f(0)
    }
}
