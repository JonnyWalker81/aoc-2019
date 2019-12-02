use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    println!("Hello, world!");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &String) -> Result<()> {
    let total = input.lines().fold(0, |acc, x| {
        let module: i64 = x.parse().expect("Expeted a number.");
        let mass = calc_fuel(module);
        acc + mass
    });

    writeln!(io::stdout(), "{}", total)?;
    Ok(())
}

fn part2(input: &String) -> Result<()> {
    let total = input.lines().fold(0, |acc, x| {
        let module: i64 = x.parse().expect("Expeted a number.");
        // let mass = rec_calc(module, 0);
        let mass = calc(module);
        acc + mass
    });

    writeln!(io::stdout(), "{}", total)?;
    Ok(())
}

fn calc_fuel(module: i64) -> i64 {
    (module / 3) - 2
}

fn calc(start: i64) -> i64 {
    let mut sum = 0;
    let mut mass = start;

    loop {
        mass = calc_fuel(mass);
        if mass <= 0 {
            break;
        }

        sum = sum + mass;
    }

    sum
}

// fn rec_calc(start: i64, acc: i64) -> i64 {
//     let mass = calc_fuel(start);
//     if mass > 0 {
//         let sum = acc + mass;
//         rec_calc(mass, sum)
//     } else {
//         acc
//     }
// }
