use std::io;
use std::io::{Read, Write};

type Result<T> = std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    println!("Hello, world!");
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &String) -> Result<()> {
    let result = execute(input, 1);
    writeln!(io::stdout(), "{}", result)?;
    Ok(())
}

fn part2(input: &String) -> Result<()> {
    let result = execute(input, 5);
    writeln!(io::stdout(), "{}", result)?;
    Ok(())
}

fn execute(input: &String, init: i64) -> i64 {
    let mut codes: Vec<i64> = input
        .split(",")
        .map(|c| c.trim().parse::<i64>().expect("Expected an i64"))
        .collect();

    let mut pc: usize = 0;
    loop {
        let current = codes[pc];

        let op = current % 100;
        match op {
            1 => {
                let pos1 = (current / 100) % 10;
                let pos2 = (current / 1000) % 10;
                let pos3 = (current / 10000) % 10;
                let first = value(pos1, pc + 1, &codes);
                let second = value(pos2, pc + 2, &codes);
                let sum = codes[first] + codes[second];
                let loc: usize = value(pos3, pc + 3, &codes) as usize;
                codes[loc] = sum;
                pc += 4;
            }
            2 => {
                let pos1 = (current / 100) % 10;
                let pos2 = (current / 1000) % 10;
                let pos3 = (current / 10000) % 10;
                let first = value(pos1, pc + 1, &codes);
                let second = value(pos2, pc + 2, &codes);
                let sum = codes[first] * codes[second];
                let loc: usize = value(pos3, pc + 3, &codes) as usize;
                codes[loc] = sum;
                pc += 4;
            }
            3 => {
                let pos1 = (current / 100) % 10;
                let loc = value(pos1, pc + 1, &codes) as usize;
                codes[loc] = init;
                pc += 2;
            }
            4 => {
                let pos1 = (current / 100) % 10;
                let loc = value(pos1, pc + 1, &codes) as usize;
                println!("{}", codes[loc]);
                pc += 2;
            }
            5 => {
                let pos1 = (current / 100) % 10;
                let pos2 = (current / 1000) % 10;
                let first = value(pos1, pc + 1, &codes);
                let second = value(pos2, pc + 2, &codes);
                if codes[first] != 0 {
                    pc = codes[second] as usize;
                } else {
                    pc += 3;
                }
            }
            6 => {
                let pos1 = (current / 100) % 10;
                let pos2 = (current / 1000) % 10;
                let first = value(pos1, pc + 1, &codes);
                let second = value(pos2, pc + 2, &codes);
                if codes[first] == 0 {
                    pc = codes[second] as usize;
                } else {
                    pc += 3;
                }
            }
            7 => {
                let pos1 = (current / 100) % 10;
                let pos2 = (current / 1000) % 10;
                let pos3 = (current / 10000) % 10;
                let first = value(pos1, pc + 1, &codes);
                let second = value(pos2, pc + 2, &codes);
                let third = value(pos3, pc + 3, &codes);
                if codes[first] < codes[second] {
                    codes[third] = 1;
                } else {
                    codes[third] = 0;
                }

                pc += 4;
            }
            8 => {
                let pos1 = (current / 100) % 10;
                let pos2 = (current / 1000) % 10;
                let pos3 = (current / 10000) % 10;
                let first = value(pos1, pc + 1, &codes);
                let second = value(pos2, pc + 2, &codes);
                let third = value(pos3, pc + 3, &codes);
                if codes[first] == codes[second] {
                    codes[third] = 1;
                } else {
                    codes[third] = 0;
                }

                pc += 4;
            }
            99 => {
                break;
            }
            _ => {
                println!("Unexpected opcode: {}", op);
                pc += 1;
            }
        }
    }

    0
}

fn value(op: i64, pc: usize, codes: &Vec<i64>) -> usize {
    match op {
        0 => codes[pc] as usize,
        1 => pc,
        _ => panic!("unexpected mode, {}", op),
    }
}
