use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

const NUM: &str = "11";

fn main() {
    let file_path = format!("/home/phobos/code/advent_2024/src/bin/{NUM}/data.txt");
    println!("{}", part(file_path.as_str()));
}

fn part(file_path: &str) -> usize {
    let mut stones = VecDeque::new();
    for line in read_to_string(file_path).unwrap().lines() {
        stones = line
            .split(' ')
            .map(|i| i.parse::<usize>().unwrap())
            .collect::<VecDeque<usize>>();
    }

    let mut map = HashMap::new();
    for stone in &stones {
        map.insert(*stone, 1);
    }

    for _ in 0..75 {
        let mut new_map = HashMap::new();

        for (&stone, count) in map.iter() {
            if stone == 0 {
                *new_map.entry(1).or_insert(0) += count;
            } else if stone.to_string().len() % 2 == 0 {
                let stone_string = stone.to_string();
                let left = stone_string[stone_string.len() / 2..stone_string.len()]
                    .parse::<usize>()
                    .unwrap();
                let right = stone_string[..stone_string.len() / 2]
                    .parse::<usize>()
                    .unwrap();
                *new_map.entry(left).or_insert(0) += count;
                *new_map.entry(right).or_insert(0) += count;
            } else {
                *new_map.entry(stone * 2024).or_insert(0) += count;
            }
        }

        println!("{:?}", new_map);
        map = new_map;
    }

    map.values().sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(
            super::part(
                format!(
                    "/home/phobos/code/advent_2024/src/bin/{}/data_test.txt",
                    super::NUM
                )
                .as_str()
            ),
            65601038650482
        );
    }
}
