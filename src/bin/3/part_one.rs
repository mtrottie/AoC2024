use std::fs;

use regex::Regex;

fn main() {
    println!(
        "{}",
        three_a("/home/phobos/code/advent_2024/src/bin/3/three.txt")
    );
}

fn three_a(file_path: &str) -> i32 {
    let message: String = fs::read_to_string(file_path).unwrap();

    // Matches mul(a,b) where a and b are 1-3 digit numbers
    // Example matches: mul(1,2), mul(123,456), mul(7,89)
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let values: Vec<&str> = re.find_iter(&message).map(|l| l.as_str()).collect();

    let mut sum = 0;
    for cap in values {
        let a = cap.split(",").collect::<Vec<&str>>()[0]
            .split("(")
            .collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();
        let b = cap.split(",").collect::<Vec<&str>>()[1]
            .split(")")
            .collect::<Vec<&str>>()[0]
            .parse::<i32>()
            .unwrap();
        sum += a * b;
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::three_a;

    #[test]
    fn test_three_a() {
        assert_eq!(
            three_a("/home/phobos/code/advent_2024/src/bin/3/three_test.txt"),
            161
        );
    }
}
