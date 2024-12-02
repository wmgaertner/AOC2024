use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part1()
}

fn part1() {
    let mut l_array: Vec<i32> = Vec::new();
    let mut r_array: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines("/home/William/projects/AOC2024/inputs/day1.txt") {
        for line in lines.flatten() {
            let mut iter = line.split_whitespace();

            l_array.push(iter.next().unwrap().parse().unwrap());
            r_array.push(iter.next().unwrap().parse().unwrap());
        }
    }

    l_array.sort_unstable();
    r_array.sort_unstable();

    let mut difference: i32 = 0;
    for i in 0..l_array.len() {
        difference += (r_array[i] - l_array[i]).abs();
    }

    println!("{}", difference);
}

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}
