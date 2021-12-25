use advent_of_code_2021::*;

aoc!(Day);

#[derive(Debug)]
enum Instr {
    Inp(Reg),
    Add(Reg, Opr),
    Mul(Reg, Opr),
    Div(Reg, Opr),
    Mod(Reg, Opr),
    Eql(Reg, Opr),
}

#[derive(Debug)]
enum Opr {
    Var(Reg),
    Imm(i64),
}

type Reg = usize;

fn parse(s: &str) -> Instr {
    let words = s.split_whitespace().collect::<Vec<_>>();

    let reg = |reg: &str| match reg {
        "w" => 0,
        "x" => 1,
        "y" => 2,
        "z" => 3,
        _ => unreachable!(),
    };

    let opr = |opr: &str| {
        if opr.chars().next().unwrap().is_alphabetic() {
            Opr::Var(reg(opr))
        } else {
            Opr::Imm(opr.parse().unwrap())
        }
    };

    use Instr::*;

    match words[..] {
        ["inp", a] => Inp(reg(a)),
        ["add", a, b] => Add(reg(a), opr(b)),
        ["mul", a, b] => Mul(reg(a), opr(b)),
        ["div", a, b] => Div(reg(a), opr(b)),
        ["mod", a, b] => Mod(reg(a), opr(b)),
        ["eql", a, b] => Eql(reg(a), opr(b)),
        _ => unreachable!(),
    }
}

struct State {
    pc: usize,
    regs: [i64; 4],
}

impl State {
    fn new() -> State {
        State {
            pc: 0,
            regs: [0; 4],
        }
    }
}

fn run(state: &mut State, progn: &[Instr], input: &[i64]) {
    use Instr::*;

    let mut input = input.iter();

    while state.pc < progn.len() {
        let instr = &progn[state.pc];
        state.pc += 1;

        match instr {
            Inp(r) => {
                state.regs[*r] = *input.next().unwrap();
            }
            Add(r, Opr::Var(x)) => state.regs[*r] += state.regs[*x],
            Add(r, Opr::Imm(x)) => state.regs[*r] += x,
            Mul(r, Opr::Var(x)) => state.regs[*r] *= state.regs[*x],
            Mul(r, Opr::Imm(x)) => state.regs[*r] *= x,
            Div(r, Opr::Var(x)) => state.regs[*r] /= state.regs[*x],
            Div(r, Opr::Imm(x)) => state.regs[*r] /= x,
            Mod(r, Opr::Var(x)) => state.regs[*r] %= state.regs[*x],
            Mod(r, Opr::Imm(x)) => state.regs[*r] %= x,
            Eql(r, Opr::Var(x)) => {
                state.regs[*r] = if state.regs[*r] == state.regs[*x] {
                    1
                } else {
                    0
                }
            }
            Eql(r, Opr::Imm(x)) => state.regs[*r] = if state.regs[*r] == *x { 1 } else { 0 },
        }
    }
}

fn verify(prog: &[Instr], input: i64) {
    let mut state = State::new();

    let mut t = input;
    let mut input = vec![];
    for _ in 0..14 {
        input.push(t % 10);
        t /= 10;
    }
    input.reverse();

    run(&mut state, prog, &input);
    assert_eq!(state.regs[3], 0);

    println!("Verify OK");
}

impl AOC for Day {
    type Input = Vec<Instr>;

    type Output = i64;

    fn input() -> Result<Self::Input> {
        let c = get_contents()?;
        Ok(c.lines().map(|line| parse(line)).collect())
    }

    fn part_one(input: &Self::Input) -> Self::Output {
        solve(input, true)
    }

    fn part_two(input: &Self::Input) -> Self::Output {
        solve(input, false)
    }
}

fn solve(progn: &[Instr], max: bool) -> i64 {
    use z3::ast::Ast;
    use z3::*;

    let mut res = vec![0; 14];

    for keta in 0..14 {
        for d in 1..=9 {
            let d = if max { 10 - d } else { d };

            let config = Config::default();
            let ctx = Context::new(&config);

            let solver = Solver::new(&ctx);

            let inps = (0..14)
                .map(|i| ast::Int::new_const(&ctx, format!("i{}", i)))
                .collect::<Vec<_>>();

            for inp in inps.iter() {
                solver.assert(&inp.ge(&ast::Int::from_i64(&ctx, 1)));
                solver.assert(&inp.le(&ast::Int::from_i64(&ctx, 9)));
            }

            res[keta] = d;

            println!("{:?}", &res);

            for i in 0..=keta {
                solver.assert(&ast::Int::from_i64(&ctx, res[i])._eq(&inps[i]));
            }

            let mut regs = (0..4)
                .map(|_| ast::Int::from_i64(&ctx, 0))
                .collect::<Vec<_>>();

            let mut input_cur = 0;

            for instr in progn {
                match instr {
                    Instr::Inp(r) => {
                        regs[*r] = inps[input_cur].clone();
                        input_cur += 1;
                    }
                    Instr::Add(r, opr) => {
                        let opr = match opr {
                            Opr::Var(x) => regs[*x].clone(),
                            Opr::Imm(x) => ast::Int::from_i64(&ctx, *x),
                        };
                        regs[*r] += opr
                    }
                    Instr::Mul(r, opr) => {
                        let opr = match opr {
                            Opr::Var(x) => regs[*x].clone(),
                            Opr::Imm(x) => ast::Int::from_i64(&ctx, *x),
                        };
                        regs[*r] *= opr
                    }
                    Instr::Div(r, opr) => {
                        let opr = match opr {
                            Opr::Var(x) => regs[*x].clone(),
                            Opr::Imm(x) => ast::Int::from_i64(&ctx, *x),
                        };
                        regs[*r] /= opr
                    }
                    Instr::Mod(r, opr) => {
                        let opr = match opr {
                            Opr::Var(x) => regs[*x].clone(),
                            Opr::Imm(x) => ast::Int::from_i64(&ctx, *x),
                        };
                        regs[*r] %= opr
                    }
                    Instr::Eql(r, opr) => {
                        let opr = match opr {
                            Opr::Var(x) => regs[*x].clone(),
                            Opr::Imm(x) => ast::Int::from_i64(&ctx, *x),
                        };
                        regs[*r] = regs[*r]
                            ._eq(&opr)
                            .ite(&ast::Int::from_i64(&ctx, 1), &ast::Int::from_i64(&ctx, 0));
                    }
                }
            }

            solver.assert(&ast::Int::from_i64(&ctx, 0)._eq(&regs[3]));
            if solver.check() == SatResult::Sat {
                let model = solver.get_model().unwrap();

                let sol = (0..14)
                    .map(|i| model.eval(&inps[i], true).unwrap().as_i64().unwrap())
                    .fold(0, |a, b| a * 10 + b);

                verify(progn, sol);
                eprintln!("Found: {}", sol);

                break;
            }
        }
    }

    res.iter().fold(0, |a, b| a * 10 + b)
}
