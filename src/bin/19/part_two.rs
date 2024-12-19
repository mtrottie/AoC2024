use std::{
    cmp,
    collections::{HashMap, HashSet},
    fs::read_to_string,
    usize,
};

const NUM: &str = "19";

fn main() {
    let file_path = format!("/home/phobos/code/advent_2024/src/bin/{NUM}/data.txt");
    println!("{}", part(file_path.as_str()));
}

fn part(file_path: &str) -> usize {
    let input = read_to_string(file_path).unwrap();
    let (patterns, designs) = input.split_once("\n\n").unwrap();
    let patterns = patterns
        .split(',')
        .map(|s| s.trim())
        .collect::<HashSet<&str>>();
    let designs = designs.split('\n').collect::<Vec<&str>>();

    println!("{patterns:?}");
    println!("{designs:?}");

    let mut total = 0;
    let mut memo = HashMap::new();
    for design in designs.into_iter() {
        total = total + min_walk(&patterns, design, &mut memo);
    }

    total
}

fn min_walk(patterns: &HashSet<&str>, design: &str, memo: &mut HashMap<String, usize>) -> usize {
    if memo.contains_key(design) {
        return *memo.get(design).unwrap();
    }

    if design.len() == 0 {
        return 1;
    }

    let mut total = 0;
    for i in 0..design.len() {
        if patterns.contains(&design[0..i + 1]) {
            total = total + min_walk(patterns, &design[i + 1..], memo);
        }
    }

    memo.insert(design.to_string(), total);

    total
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
            16
        );
    }
}
