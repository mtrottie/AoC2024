use std::fs::read_to_string;

fn main() {
    println!(
        "{}",
        two_a("/home/phobos/code/advent_2024/src/bin/2/two.txt")
    );
}

fn two_a(file_path: &str) -> i32 {
    let mut safe = 0;

    for line in read_to_string(file_path).unwrap().lines() {
        let report = line
            .split_whitespace()
            .into_iter()
            .map(|i| i.parse::<i32>().unwrap())
            .collect();

        if is_desc_safe(&report) || is_asc_safe(&report) {
            safe = safe + 1;
        }
    }

    safe
}

fn is_desc_safe(report: &Vec<i32>) -> bool {
    for i in 0..report.len() - 1 {
        if report[i] >= report[i + 1] || report[i + 1] - report[i] > 3 {
            return false;
        }
    }

    true
}

fn is_asc_safe(report: &Vec<i32>) -> bool {
    for i in 0..report.len() - 1 {
        if report[i] <= report[i + 1] || report[i] - report[i + 1] > 3 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::two_a;

    #[test]
    fn test_two_a() {
        assert_eq!(
            two_a("/home/phobos/code/advent_2024/src/bin/2/two.test.txt"),
            2
        );
    }
}
