use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::io;
use std::io::{Read, Write};
use std::str;

type Result<T> = std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    println!("Hello, world!");
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

#[derive(Debug)]
enum ErrorParsing {
    InvalidOrbit,
}

#[derive(Debug, Clone)]
struct Orbit {
    from: String,
    to: String,
}

// #[derive(Debug)]
// struct OrbitNode {
//     parent: Option<Arc<OrbitNode>>,
//     children: Vec<OrbitNode>,
// }

// impl OrbitNode {
//     fn new() -> Self {
//         OrbitNode {
//             parent: None,
//             children: Vec::new(),
//         }
//     }
// }

impl TryFrom<&str> for Orbit {
    type Error = ErrorParsing;

    fn try_from(input: &str) -> std::result::Result<Self, Self::Error> {
        let parts: Vec<&str> = input.split(")").collect();

        if parts.len() > 1 {
            Ok(Orbit {
                from: parts[0].into(),
                to: parts[1].into(),
            })
        } else {
            Err(ErrorParsing::InvalidOrbit)
        }
    }
}

fn part1(input: &String) -> Result<()> {
    let tree = build_tree(input, true);

    let mut sum = 0;
    for (k, _) in &tree {
        sum += count_nodes(&tree, &k);
    }

    println!("Sum: {}", sum);

    Ok(())
}

fn count_nodes(tree: &HashMap<String, Vec<String>>, key: &String) -> i64 {
    let mut sum: i64 = 0;
    if tree.contains_key(key) {
        let v = &tree[key];
        for k in v {
            sum += v.len() as i64;
            sum += count_nodes(tree, &k);
        }
    }

    sum
}

fn part2(input: &String) -> Result<()> {
    let tree = build_tree(input, true);
    // println!("Orbits: {:#?}", tree);
    let mut you_acc = Vec::new();
    let mut san_acc = Vec::new();
    dfs(&tree, &"YOU".into(), &"COM".into(), &mut you_acc);
    dfs(&tree, &"SAN".into(), &"COM".into(), &mut san_acc);

    let mut t = 0;
    for (i, p) in you_acc.iter().enumerate() {
        if san_acc.contains(&p) {
            t = (i as i32)
                + san_acc
                    .iter()
                    .position(|r| r.cmp(&p) == std::cmp::Ordering::Equal)
                    .unwrap() as i32;
            break;
        }
    }

    println!("Hops: {}", t);
    Ok(())
}

fn dfs(tree: &HashMap<String, Vec<String>>, from: &String, to: &String, acc: &mut Vec<String>) {
    match tree.get(from) {
        Some(n) => dfs_inner(tree, n, to, acc),
        None => {}
    }
}

fn dfs_inner(
    tree: &HashMap<String, Vec<String>>,
    children: &Vec<String>,
    to: &String,
    acc: &mut Vec<String>,
) {
    for k in children {
        if k == to {
            acc.push(k.clone());
            break;
        }

        match tree.get(k) {
            Some(n) => {
                // println!("Node: {}", k);
                acc.push(k.clone());
                dfs_inner(tree, n, to, acc);
            }
            None => {}
        };
    }
}

fn build_tree(input: &String, direction: bool) -> HashMap<String, Vec<String>> {
    let orbits: Vec<Orbit> = input
        .split("\n")
        .filter(|l| l.len() != 0)
        .map(|l| {
            let orbit: Orbit = l
                .trim()
                .try_into()
                .expect(format!("Expected an Orbit: {}", l).as_str());
            orbit
        })
        .collect();

    // let tree: HashMap<String, Vec<String>> = HashMap::new();

    let tree: HashMap<String, Vec<String>> = orbits.iter().fold(HashMap::new(), |mut acc, o| {
        acc.entry(o.to.clone())
            .and_modify(|v| v.push(o.from.clone()))
            .or_insert(vec![o.from.clone()]);
        acc
    });

    tree
}
