use std::{collections::HashMap, fs::read_to_string};

const NUM: &str = "5";

fn main() {
    let file_name = format!("/home/phobos/code/advent_2024/src/bin/{NUM}/data.txt");
    println!("{}", part(&file_name));
}

fn part(file_path: &str) -> i32 {
    let mut adj_map = HashMap::new();
    let mut rules: Vec<Vec<i32>> = Vec::new();
    let binding = read_to_string(file_path).unwrap();

    for line in binding.lines() {
        if line.contains('|') {
            let (k, v) = line.split_once('|').unwrap();
            let k = k.parse::<i32>().unwrap();
            let v = v.parse::<i32>().unwrap();

            if !adj_map.contains_key(&k) {
                adj_map.insert(k, vec![v]);
            } else {
                adj_map.get_mut(&k).unwrap().push(v);
            }
        } else if !line.is_empty() {
            rules.push(
                line.split(',')
                    .map(|i| i.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            );
        }
    }

    println!("{:?}", adj_map);
    println!("{:?}", rules);

    let mut count = 0;
    for rule in rules.into_iter() {
        let mut start = rule[0];

        for i in 1..rule.len() {
            if adj_map.contains_key(&start) && adj_map.get(&start).unwrap().contains(&rule[i]) {
                start = rule[i];
            } else {
                break;
            }
        }

        if start == rule[rule.len() - 1] {
            count = count + rule[rule.len() / 2];
        }
    }

    count
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
            143
        );
    }
}
