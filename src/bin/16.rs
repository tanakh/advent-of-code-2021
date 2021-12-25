use std::fmt::Display;

use advent_of_code_2021::*;

aoc!(Day);

impl AOC for Day {
    type Input = String;

    type Output = u64;

    fn input() -> Result<Self::Input> {
        Ok(get_contents()?.trim().to_string())
    }

    fn part_one(input: &Self::Input) -> Self::Output {
        let mut rdr = BitStream::new(input);
        let packet = parse(&mut rdr);
        assert!(rdr.is_empty());
        eprintln!("{}", packet);
        packet.sum()
    }

    fn part_two(input: &Self::Input) -> Self::Output {
        let mut rdr = BitStream::new(input);
        let packet = parse(&mut rdr);
        assert!(rdr.is_empty());
        packet.eval()
    }
}

#[derive(Debug)]
struct Packet {
    version: u64,
    data: PacketData,
}

#[derive(Debug)]
enum PacketData {
    Literal(u64),
    Operator { type_id: u64, subs: Vec<Packet> },
}

impl Packet {
    fn sum(&self) -> u64 {
        self.version + self.data.sum()
    }

    fn eval(&self) -> u64 {
        self.data.eval()
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "[{}]", self.version)?;
        match &self.data {
            PacketData::Literal(v) => {
                write!(f, "{}", v)?;
            }
            PacketData::Operator { type_id, subs } => {
                write!(
                    f,
                    "({}",
                    match *type_id {
                        0 => "sum",
                        1 => "prod",
                        2 => "min",
                        3 => "max",
                        5 => "<",
                        6 => ">",
                        7 => "=",
                        _ => unreachable!(),
                    }
                )?;
                for sub in subs {
                    write!(f, " {}", sub)?;
                }
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}

impl PacketData {
    fn sum(&self) -> u64 {
        match self {
            PacketData::Literal(_) => 0,
            PacketData::Operator { subs, .. } => subs.iter().map(|p| p.sum()).sum(),
        }
    }

    fn eval(&self) -> u64 {
        match self {
            PacketData::Literal(v) => *v,
            PacketData::Operator { type_id, subs } => {
                let mut evals = subs.iter().map(|p| p.eval());

                match *type_id {
                    0 => evals.sum(),
                    1 => evals.product(),
                    2 => evals.min().unwrap(),
                    3 => evals.max().unwrap(),

                    5 | 6 | 7 => {
                        let l = evals.next().unwrap();
                        let r = evals.next().unwrap();
                        assert!(evals.next().is_none());

                        let cond = match *type_id {
                            5 => l < r,
                            6 => l > r,
                            7 => l == r,
                            _ => unreachable!(),
                        };

                        if cond {
                            1
                        } else {
                            0
                        }
                    }

                    _ => unreachable!(),
                }
            }
        }
    }
}

fn parse(rdr: &mut BitStream) -> Packet {
    let version = rdr.get(3);
    let type_id = rdr.get(3);

    let data = if type_id == 4 {
        let mut v = 0;

        loop {
            let last = rdr.get(1) == 0;
            v = v * 16 + rdr.get(4);
            if last {
                break;
            }
        }

        PacketData::Literal(v)
    } else {
        let mut subs = vec![];

        let len_type_id = rdr.get(1);

        if len_type_id == 0 {
            let bits = rdr.get(15) as usize;
            let start = rdr.pos();

            while rdr.pos() - start < bits {
                subs.push(parse(rdr));
            }

            assert_eq!(rdr.pos() - start, bits);
        } else {
            let npack = rdr.get(11);
            for _ in 0..npack {
                subs.push(parse(rdr));
            }
        }

        PacketData::Operator { type_id, subs }
    };

    Packet { version, data }
}

struct BitStream {
    buf: Vec<u8>,
    pos: usize,
}

impl BitStream {
    fn new(s: &str) -> Self {
        Self {
            buf: s.chars().map(|c| c.to_digit(16).unwrap() as u8).collect(),
            pos: 0,
        }
    }

    fn get_bit(&mut self) -> u8 {
        let ret = if self.pos >= self.buf.len() * 4 {
            0
        } else {
            (self.buf[self.pos / 4] >> (3 - self.pos % 4)) & 1
        };

        self.pos += 1;
        ret
    }

    fn get(&mut self, n: usize) -> u64 {
        let mut ret = 0;
        for _ in 0..n {
            ret = ret * 2 + self.get_bit() as u64;
        }
        ret
    }

    fn is_empty(&mut self) -> bool {
        while self.pos < self.buf.len() * 4 {
            if self.get_bit() == 1 {
                return false;
            }
        }
        true
    }

    fn pos(&self) -> usize {
        self.pos
    }
}
