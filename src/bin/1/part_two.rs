use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let mut column_one = Vec::new();
    let mut map = HashMap::new();

    for line in read_to_string("/home/phobos/code/advent_2024/src/bin/1/one_b.txt")
        .unwrap()
        .lines()
    {
        let mut iter = line.split_whitespace();
        let a = iter.next().unwrap();
        let b = iter.next_back().unwrap();

        println!("{} {}", a, b);
        let a = a.parse::<i32>().unwrap();
        let b = b.parse::<i32>().unwrap();

        column_one.push(a);
        map.insert(b, map.get(&b).unwrap_or(&0) + 1);
    }

    let mut count = 0;
    for item in column_one {
        count = count + item * map.get(&item).unwrap_or(&0);
    }

    println!("{}", count);
}
