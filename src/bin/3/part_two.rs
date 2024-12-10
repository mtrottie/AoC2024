use std::fs;

use regex::Regex;

fn main() {
    println!(
        "{}",
        three_b("/home/phobos/code/advent_2024/src/bin/3/three.txt")
    );
}

fn three_b(file_path: &str) -> i64 {
    let message: String = fs::read_to_string(file_path).unwrap();
    let message = truncate_message(&message);
    let message_two = truncate_message_v2(&message);
    assert_eq!(message, message_two);

    // Matches mul(a,b) where a and b are 1-3 digit numbers
    // Example matches: mul(1,2), mul(123,456), mul(7,89)
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let values: Vec<&str> = re.find_iter(&message).map(|l| l.as_str()).collect();

    let mut sum: i64 = 0;
    for cap in values {
        let a = cap.split(",").collect::<Vec<&str>>()[0]
            .split("(")
            .collect::<Vec<&str>>()[1]
            .parse::<i64>()
            .unwrap();
        let b = cap.split(",").collect::<Vec<&str>>()[1]
            .split(")")
            .collect::<Vec<&str>>()[0]
            .parse::<i64>()
            .unwrap();
        sum += a * b;
    }

    sum
}

fn truncate_message_v2(message: &str) -> String {
    let mut new_message = String::new();

    let mut i = 0;
    while i < message.len() {
        let index: Option<usize> = message[i..].find("don't()");
        match index {
            Some(index) => {
                if index > i {
                    new_message += &message[i..index].to_string();
                }

                let index_do: Option<usize> = message[index..].find("do()");

                match index_do {
                    Some(index_do) => {
                        i = index_do;
                    }
                    None => {
                        i = message.len();
                    }
                }
            }
            None => {
                new_message += &message[i..].to_string();
                i = message.len();
            }
        }
    }

    new_message
}

fn truncate_message(message: &str) -> String {
    match message.split_once("don't()") {
        Some((left_do, right_do_not)) => match right_do_not.split_once("do()") {
            Some((_, right_do)) => {
                let right_do = truncate_message(right_do);
                left_do.to_owned() + &right_do
            }
            None => left_do.to_owned(),
        },
        None => message.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::three_b;

    #[test]
    fn test_three_b() {
        assert_eq!(
            three_b("/home/phobos/code/advent_2024/src/bin/3/three_b_test.txt"),
            8
        );
    }
}
