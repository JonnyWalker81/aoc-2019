use itertools::Itertools;
use std::io;
use std::io::{Read, Write};

type Result<T> = std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    println!("Hello, world!");
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &String) -> Result<()> {
    let mut signals = vec![];
    let mut amp_input = 0;
    let perms = (0..=4).permutations(5);
    println!("Perms: {:#?}", perms);

    for v in perms {
        println!("v: {:?}", v);
        amp_input = 0;
        for p in v {
            let mut interpreter = Amp::new(input);
            amp_input = interpreter.execute(Some(p), amp_input).unwrap();
        }
        signals.push(amp_input);
    }

    let max = signals.iter().max().unwrap_or(&0);
    writeln!(io::stdout(), "Max: {}", max)?;
    Ok(())
}

fn part2(input: &String) -> Result<()> {
    let mut signals = vec![];
    let mut amp_input = 0;
    let perms = (5..=9).permutations(5);
    // println!("Perms: {:#?}", perms);

    // let mut flat_perms = vec![];
    // for v in perms {
    //     for p in v {
    //         flat_perms.push(p);
    //     }
    // }

    // println!("Persm: {} -- {:?}", flat_perms.len(), flat_perms);
    // let mut in_iter = flat_perms.iter().cloned();
    for v in perms {
        let mut in_iter = v.iter().cloned();
        amp_input = 0;
        let mut amps = vec![Amp::new(input); 5];
        for i in (0..amps.len()).cycle() {
            if let Some(ai) = amps[i].execute(in_iter.next(), amp_input) {
                amp_input = ai;
            } else {
                // halt = true;
                break;
            }
        }

        signals.push(amp_input);
    }
    let max = signals.iter().max().unwrap_or(&0);
    writeln!(io::stdout(), "Max: {}", max)?;
    Ok(())
}

#[derive(Clone)]
struct Amp {
    pc: usize,
    codes: Vec<i64>,
}

impl Amp {
    fn new(input: &String) -> Self {
        Self {
            codes: input
                .split(",")
                .map(|c| c.trim().parse::<i64>().expect("Expected an i64"))
                .collect(),
            pc: 0,
        }
    }

    fn execute(&mut self, mut phase: Option<i64>, init: i64) -> Option<i64> {
        // let mut codes: Vec<i64> = input
        //     .split(",")
        //     .map(|c| c.trim().parse::<i64>().expect("Expected an i64"))
        //     .collect();

        // let mut self.pc: usize = 0;

        // let mut is_init = true;
        let mut output = 0;
        loop {
            let current = self.codes[self.pc];

            let op = current % 100;
            match op {
                1 => {
                    let pos1 = (current / 100) % 10;
                    let pos2 = (current / 1000) % 10;
                    let pos3 = (current / 10000) % 10;
                    let first = value(pos1, self.pc + 1, &self.codes);
                    let second = value(pos2, self.pc + 2, &self.codes);
                    let sum = self.codes[first] + self.codes[second];
                    let loc: usize = value(pos3, self.pc + 3, &self.codes) as usize;
                    self.codes[loc] = sum;
                    self.pc += 4;
                }
                2 => {
                    let pos1 = (current / 100) % 10;
                    let pos2 = (current / 1000) % 10;
                    let pos3 = (current / 10000) % 10;
                    let first = value(pos1, self.pc + 1, &self.codes);
                    let second = value(pos2, self.pc + 2, &self.codes);
                    let sum = self.codes[first] * self.codes[second];
                    let loc: usize = value(pos3, self.pc + 3, &self.codes) as usize;
                    self.codes[loc] = sum;
                    self.pc += 4;
                }
                3 => {
                    let pos1 = (current / 100) % 10;
                    let loc = value(pos1, self.pc + 1, &self.codes) as usize;
                    // println!("phase: {}", phase);
                    if let Some(v) = phase.take() {
                        self.codes[loc] = v;
                    // self.is_init = false;
                    } else {
                        self.codes[loc] = init;
                    }
                    self.pc += 2;
                }
                4 => {
                    let pos1 = (current / 100) % 10;
                    let loc = value(pos1, self.pc + 1, &self.codes) as usize;
                    println!("{}", self.codes[loc]);
                    output = self.codes[loc];
                    self.pc += 2;
                    return Some(output);
                }
                5 => {
                    let pos1 = (current / 100) % 10;
                    let pos2 = (current / 1000) % 10;
                    let first = value(pos1, self.pc + 1, &self.codes);
                    let second = value(pos2, self.pc + 2, &self.codes);
                    if self.codes[first] != 0 {
                        self.pc = self.codes[second] as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                6 => {
                    let pos1 = (current / 100) % 10;
                    let pos2 = (current / 1000) % 10;
                    let first = value(pos1, self.pc + 1, &self.codes);
                    let second = value(pos2, self.pc + 2, &self.codes);
                    if self.codes[first] == 0 {
                        self.pc = self.codes[second] as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                7 => {
                    let pos1 = (current / 100) % 10;
                    let pos2 = (current / 1000) % 10;
                    let pos3 = (current / 10000) % 10;
                    let first = value(pos1, self.pc + 1, &self.codes);
                    let second = value(pos2, self.pc + 2, &self.codes);
                    let third = value(pos3, self.pc + 3, &self.codes);
                    if self.codes[first] < self.codes[second] {
                        self.codes[third] = 1;
                    } else {
                        self.codes[third] = 0;
                    }

                    self.pc += 4;
                }
                8 => {
                    let pos1 = (current / 100) % 10;
                    let pos2 = (current / 1000) % 10;
                    let pos3 = (current / 10000) % 10;
                    let first = value(pos1, self.pc + 1, &self.codes);
                    let second = value(pos2, self.pc + 2, &self.codes);
                    let third = value(pos3, self.pc + 3, &self.codes);
                    if self.codes[first] == self.codes[second] {
                        self.codes[third] = 1;
                    } else {
                        self.codes[third] = 0;
                    }

                    self.pc += 4;
                }
                99 => {
                    break;
                }
                _ => {
                    println!("Unexpected oself.pcode: {}", op);
                    self.pc += 1;
                }
            }
        }

        None
    }
}

fn value(op: i64, pc: usize, codes: &Vec<i64>) -> usize {
    match op {
        0 => codes[pc] as usize,
        1 => pc,
        _ => panic!("unexpected mode, {}", op),
    }
}
