use std::collections::{HashMap, HashSet};
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
    let vals: Vec<i64> = input
        .split("-")
        .map(|v| v.trim().parse().expect("Expected a number"))
        .collect();
    let range = (vals[0], vals[1]);

    let mut matches: HashSet<i64> = HashSet::new();

    for i in range.0..range.1 {
        let has_adj = has_adjacent(i);
        let is_inc = is_increasing(i);

        if has_adj && is_inc {
            matches.insert(i);
        }
    }

    let _ = writeln!(io::stdout(), "Matches Len: {}\n", matches.len());
    Ok(())
}

fn part2(input: &String) -> Result<()> {
    let vals: Vec<i64> = input
        .split("-")
        .map(|v| v.trim().parse().expect("Expected a number"))
        .collect();
    let range = (vals[0], vals[1]);

    let mut matches: HashSet<i64> = HashSet::new();

    for i in range.0..range.1 {
        let has_adj = has_adjacent_count(i);
        let is_inc = is_increasing(i);

        if has_adj && is_inc {
            matches.insert(i);
        }
    }

    let _ = writeln!(io::stdout(), "Matches Len: {}\n", matches.len());
    Ok(())
}

fn has_adjacent_count(i: i64) -> bool {
    let mut mapping: HashMap<i64, i64> = HashMap::new();
    let mut modifier = 100_000;
    loop {
        let d = (i / modifier) % 10;
        mapping
            .entry(d)
            .and_modify(|v| {
                *v += 1;
            })
            .or_insert(1);

        modifier /= 10;
        if modifier <= 0 {
            break;
        }
    }

    let groups: HashSet<i64> = mapping.iter().map(|(_, v)| *v).collect();

    if groups.contains(&2) {
        return true;
    }

    false
}

fn has_adjacent(i: i64) -> bool {
    let mut modifier = 100_000;
    let mut cur = (i / modifier) % 10;
    modifier /= 10;
    loop {
        let d = (i / modifier) % 10;
        if cur == d {
            return true;
        }

        cur = d;
        modifier /= 10;
        if modifier <= 0 {
            break;
        }
    }

    false
}

fn is_increasing(i: i64) -> bool {
    let mut modifier = 100_000;
    let mut cur = (i / modifier) % 10;
    modifier /= 10;
    loop {
        let d = (i / modifier) % 10;
        if cur > d {
            return false;
        }

        cur = d;

        modifier /= 10;
        if modifier <= 0 {
            break;
        }
    }

    true
}
