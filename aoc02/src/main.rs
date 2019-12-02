use std::io;
use std::io::{Read, Write};

type Result<T> = std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()>{
    println!("Hello, world!");
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &String) -> Result<()>{
    let result = execute(input, 12, 2);
    writeln!(io::stdout(), "{}", result)?;
    Ok(())
}

fn part2(input: &String) -> Result<()> {
    let mut noun = 0;
    let mut verb = 0;
    
    for n in 0..=99 {
        for v in 0..=99 {
            let result = execute(input, n, v);

            if result == 19690720 {
                noun =n;
                verb = v;
                break;
            }
        }
    }

    let gravity = 100 * noun + verb;
    writeln!(io::stdout(), "{}", gravity)?;
    Ok(())
}

fn execute(input: &String, noun: i64, verb: i64) -> i64 {
    let mut codes: Vec<i64> = input.split(",").map(|c| c.trim().parse::<i64>().expect("Expected an i64")).collect();
    // println!("Codes: {:#?}", codes);
    codes[1] = noun;
    codes[2] = verb;
    
    let mut index = 0;
    // let operand_index = 0;
    loop {
        let current = codes[index];
        match current {
            1 => {
                let first = codes[index + 1] as usize;
                let second = codes[index + 2] as usize;
                let sum = codes[first] + codes[second];
                let loc: usize = codes[index + 3] as usize;
                // println!("Index: {}", index);
                // println!("First: {}", first);
                // println!("Second: {}", second);
                // println!("Sum: {}", sum);
                codes[loc] = sum;
            },
            2 => {
                let first = codes[index + 1] as usize;
                let second = codes[index + 2] as usize;
                let sum = codes[first] * codes[second];
                let loc: usize = codes[index + 3] as usize;
                codes[loc] = sum;
            },
            99 => {
                break;
            },
            _ => panic!("Unexpected Opcode.")
        }

        index += 4;
    }

    codes[0]
}
