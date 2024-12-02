use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    part1();
    part2();
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

fn part2() {
    let mut l_array: Vec<i32> = Vec::new();
    let mut r_map: HashMap<i32, i32> = HashMap::new();

    if let Ok(lines) = read_lines("/home/William/projects/AOC2024/inputs/day1.txt") {
        for line in lines.flatten() {
            let mut iter = line.split_whitespace();

            let left: i32 = iter.next().unwrap().parse().unwrap();
            let right: i32 = iter.next().unwrap().parse().unwrap();

            l_array.push(left);
            let similarity = r_map.entry(right).or_insert(0);
            *similarity += 1;
            
        }
    }
    let mut similarity_score: i32 = 0;
    for num in l_array {
        match r_map.get(&num) {
            Some(x) => similarity_score += num * x,
            None => ()
        }
    }

    println!("{}", similarity_score);
}
fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}
