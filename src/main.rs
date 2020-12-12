use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::cmp::min;


fn main() {
    let lines: Vec<String> = read_file("input.txt");
    let unsorted_adapters: Vec<i64> = to_ints(lines);
    let sorted_adapters: Vec<i64> = sorted_with_ends(&unsorted_adapters);
    let jumps = find_jumps(&sorted_adapters);
    let answer1 = task1(&jumps);
    println!("Answer 1: {} ", answer1);
    let mut cache: HashMap<usize, i64> = HashMap::new();
    let answer2 = paths(&sorted_adapters, 0, &mut cache);
    println!("Answer 2: {} ", answer2);
}

fn task1(jumps: &Vec<i64>) -> usize {
    let ones = (&jumps).into_iter().filter(|n| n.clone().clone() == 1).count();
    let threes = (&jumps).into_iter().filter(|n| n.clone().clone() == 3).count();
    ones * threes
}

fn can_reach(adapters: &Vec<i64>, pos: usize, available: usize) -> usize {
    (0..available)
        .map(|i| adapters[pos+i+1]-adapters[pos])
        .filter(|n| n < &4)
        .count()
}

fn paths(adapters: &Vec<i64>, pos: usize, cache: &mut HashMap<usize, i64>) -> i64 {
    let at_end = pos == (adapters.len()-1);
    if at_end {
        println!("At end: [{}]",adapters[pos]);
        1
    } else {
        if cache.contains_key(&pos) {
            cache[&pos]
        } else {
            let remaining_adapters = adapters.len() - pos - 1;
            let available = min(3, remaining_adapters);
            let in_range = can_reach(adapters, pos, available);
            println!("pos={} available={} in_range={}", pos, available, in_range);
            let mut value = 0;
            for i in 1..in_range + 1 {
                let next = pos + i;
                println!("{} => {}", adapters[pos], adapters[next]);
                value += paths(adapters, next, cache);
            }
            cache.insert(pos.clone(), value);
            value
        }
    }
}

fn find_jumps(adapters: &Vec<i64>) -> Vec<i64> {
    let mut jumps: Vec<i64> = vec!();
    for i in 0..adapters.len() - 1 {
        println!("{} -> {} = {}", adapters[i], adapters[i + 1], adapters[i + 1] - adapters[i]);
        jumps.push(adapters[i + 1] - adapters[i]);
    }
    jumps
}

fn sorted_with_ends(unsorted: &Vec<i64>) -> Vec<i64> {
    let mut ints = unsorted.clone();
    ints.sort();
    ints.insert(0, 0);
    ints.push(ints.last().unwrap().clone() + 3);
    ints
}

fn to_ints(lines: Vec<String>) -> Vec<i64> {
    lines.into_iter()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn read_file(filename: &str) -> Vec<String> {
    let file = File::open(filename)
        .expect(format!("File not found: {}", filename).as_str());
    let reader = BufReader::new(file);
    reader.lines().map(|s| s.expect("Could not collect line")).collect()
}