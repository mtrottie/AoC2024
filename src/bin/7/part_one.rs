use std::fs::read_to_string;

const NUM: &str = "7";

fn main() {
    let file_path = format!("/home/phobos/code/advent_2024/src/bin/{NUM}/data.txt");
    println!("{}", part(file_path.as_str()));
}

fn part(file_path: &str) -> u128 {
    let mut count = 0;
    for line in read_to_string(file_path).unwrap().lines() {
        let (value, numbers) = line.split_once(": ").unwrap();
        let target_value = value.parse::<usize>().unwrap();
        let numbers = numbers
            .split_whitespace()
            .into_iter()
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        if walk(&numbers, 0, target_value as u128, 0) {
            count = count + target_value as u128;
        }
    }

    count
}

fn walk(numbers: &Vec<usize>, start_index: usize, target: u128, value: u128) -> bool {
    if target == value && start_index == numbers.len() {
        return true;
    }

    if start_index >= numbers.len() {
        return false;
    }

    walk(
        numbers,
        start_index + 1,
        target,
        numbers[start_index] as u128 + value,
    ) || walk(
        numbers,
        start_index + 1,
        target,
        numbers[start_index] as u128 * value,
    ) || walk(
        numbers,
        start_index + 1,
        target,
        (value.to_string() + &numbers[start_index].to_string())
            .parse::<u128>()
            .unwrap(),
    )
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
            11387
        );
    }
}
