use std::fs::read_to_string;

fn main() {
    println!(
        "{}",
        two_b("/home/phobos/code/advent_2024/src/bin/2/two_b.txt")
    );
}

fn two_b(file_path: &str) -> i32 {
    let mut safe = 0;

    for line in read_to_string(file_path).unwrap().lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .into_iter()
            .map(|i| i.parse::<i32>().unwrap())
            .collect();

        if is_desc_safe(&report) || is_asc_safe(&report) {
            safe = safe + 1;
        } else {
            if is_desc_safe_iter(&report) || is_asc_safe_iter(&report) {
                safe = safe + 1;
            }
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

fn is_desc_safe_iter(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let concat = &[&report[0..i], &report[i + 1..report.len()]].concat();

        if is_desc_safe(concat) {
            return true;
        }
    }

    return false;
}

fn is_asc_safe_iter(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let concat = [&report[0..i], &report[i + 1..report.len()]].concat();

        if is_asc_safe(&concat) {
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use crate::two_b;

    #[test]
    fn test_two_b() {
        assert_eq!(
            two_b("/home/phobos/code/advent_2024/src/bin/2/two_b_test.txt"),
            6
        );
    }
}
