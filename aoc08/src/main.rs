use std::convert::{TryFrom, TryInto};
use std::io;
use std::io::{Read, Write};
use std::str::FromStr;

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

impl From<std::str::Utf8Error> for ErrorParsing {
    fn from(_err: std::str::Utf8Error) -> Self {
        ErrorParsing::InvalidOrbit
    }
}

#[derive(Debug, Clone)]
struct Layer {
    data: Vec<Vec<i64>>,
}

impl Layer {
    fn new() -> Self {
        Self { data: vec![] }
    }
}

impl FromStr for Layer {
    // type Error = ErrorParsing;
    type Err = ErrorParsing;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        let int_vec: Vec<i64> = input
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .expect(format!("Expected a number: {}", c).as_str()) as i64
            })
            .collect();
        let layer: Vec<Vec<i64>> = int_vec.chunks(ROW).map(|i| i.to_vec()).collect();

        if layer.len() > 0 {
            Ok(Layer { data: layer })
        } else {
            Err(ErrorParsing::InvalidOrbit)
        }
    }
}

const ROW: usize = 25;
const COL: usize = 6;

fn count_occurances_2d(data: &Vec<Vec<i64>>, key: i64) -> i64 {
    let mut count = 0;
    for i in data {
        for j in i {
            if *j == key {
                count += 1;
            }
        }
    }

    count
}

fn count_occurances(data: &Vec<i64>, key: i64) -> i64 {
    let mut count = 0;
    for i in data {
        if *i == key {
            count += 1;
        }
    }

    count
}

fn part1(input: &String) -> Result<()> {
    let layers: Vec<Layer> = input
        .trim()
        .as_bytes()
        .chunks(ROW * COL)
        .map(std::str::from_utf8)
        .map(|s| s.unwrap().parse().expect("Invalid Layer"))
        .collect();

    // println!("Layers: {:#?}", layers);

    let fewest_zero_layer = layers
        .iter()
        .min_by_key(|l| count_occurances_2d(&l.data, 0))
        .expect("Expected a min");

    let ones_count = fewest_zero_layer.data.iter().fold(0, |mut acc, l| {
        acc += count_occurances(&l, 1);
        acc
    });

    let twos_count = fewest_zero_layer.data.iter().fold(0, |mut acc, l| {
        acc += count_occurances(&l, 2);
        acc
    });

    // println!("Min Layer: {:?}", fewest_zero_layer);

    // println!("{} -- {}", ones_count, twos_count);
    let result = ones_count * twos_count;

    println!("Result: {}", result);

    Ok(())
}

fn part2(input: &String) -> Result<()> {
    let layers: Vec<Layer> = input
        .trim()
        .as_bytes()
        .chunks(ROW * COL)
        .map(std::str::from_utf8)
        .map(|s| s.unwrap().parse().expect("Invalid Layer"))
        .collect();

    let mut image: Vec<Vec<i64>> = vec![vec![2; ROW]; COL];

    // println!("Initial Image: {:#?}", image);

    for i in 0..ROW {
        for j in 0..COL {
            for l in &layers {
                let c = l.data[j][i];
                if c != 2 {
                    // println!("Color: {}", c);
                    image[j][i] = c;
                    break;
                }
            }
        }
    }

    for r in &image {
        for c in r {
            print!("{}", c);
        }

        println!();
    }

    // println!("Image: {:#?}", image);

    Ok(())
}
