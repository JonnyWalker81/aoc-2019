use std::collections::{HashMap, HashSet};
use std::io;
use std::io::{Read, Write};

type Result<T> = std::result::Result<T, Box<dyn ::std::error::Error>>;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i64,
    y: i64,
}

fn main() -> Result<()> {
    println!("Hello, world!");
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &String) -> Result<()> {
    let mut duplicates: HashSet<i64> = HashSet::new();
    let lines: Vec<String> = input
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(String::from)
        .collect();

    if lines.len() == 2 {
        let first_wire: HashSet<Point> = collect_points(&lines[0])
            .iter()
            .map(|(k, _)| k.clone())
            .collect();
        let second_wire: HashSet<Point> = collect_points(&lines[1])
            .iter()
            .map(|(k, _)| k.clone())
            .collect();

        let intersection = first_wire.intersection(&second_wire);
        for i in intersection {
            let man = i.x.abs() + i.y.abs();
            duplicates.insert(man);
        }
    }

    if duplicates.len() > 0 {
        let min = duplicates.iter().min().expect("Expected Min Value");
        let _ = writeln!(io::stdout(), "{}\n", min);
    }
    Ok(())
}

fn part2(input: &String) -> Result<()> {
    let mut duplicates: HashSet<i64> = HashSet::new();
    let lines: Vec<String> = input
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(String::from)
        .collect();

    if lines.len() == 2 {
        let first_wire = collect_points(&lines[0]);
        let second_wire = collect_points(&lines[1]);

        for (k, v) in second_wire {
            if first_wire.contains_key(&k) {
                let first_v = first_wire[&k];
                let man = v + first_v;
                duplicates.insert(man);
            }
        }
    }

    if duplicates.len() > 0 {
        let min = duplicates.iter().min().expect("Expected Min Value");
        let _ = writeln!(io::stdout(), "{}\n", min);
    }
    Ok(())
}

fn collect_points(line: &str) -> HashMap<Point, i64> {
    let mut visited: HashMap<Point, i64> = HashMap::new();
    let mut current = Point { x: 0, y: 0 };
    let mut step = 0;
    let directions: Vec<(char, i64)> = line
        .split(",")
        .map(|l| {
            let dir: char = l[0..1].parse().expect("Expected a char");
            let rest: i64 = l[1..].parse().expect("Expected a number");
            (dir, rest)
        })
        .collect();
    for d in directions {
        match d.0 {
            'R' => {
                for _ in 0..d.1 {
                    let next = Point {
                        x: current.x + 1,
                        y: current.y,
                    };
                    current = next.clone();
                    step += 1;
                    visited.insert(next, step);
                }
            }
            'L' => {
                for _ in 0..d.1 {
                    current = Point {
                        x: current.x - 1,
                        y: current.y,
                    };
                    step += 1;
                    visited.insert(current.clone(), step);
                }
            }
            'U' => {
                for _ in 0..d.1 {
                    current = Point {
                        x: current.x,
                        y: current.y + 1,
                    };
                    step += 1;
                    visited.insert(current.clone(), step);
                }
            }
            'D' => {
                for _ in 0..d.1 {
                    current = Point {
                        x: current.x,
                        y: current.y - 1,
                    };
                    step += 1;
                    visited.insert(current.clone(), step);
                }
            }
            _ => panic!("Unexpected Direction"),
        }
    }
    visited
}
