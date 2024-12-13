use std::{cmp, fs::read_to_string};

const NUM: &str = "13";

fn main() {
    let file_path = format!("/home/phobos/code/advent_2024/src/bin/{NUM}/data.txt");
    println!("{}", part(file_path.as_str()));
}

#[derive(Debug)]
struct Game {
    a: (i32, i32),
    b: (i32, i32),
    prize: (i32, i32),
}

fn part(file_path: &str) -> usize {
    let mut games = Vec::new();

    let mut a = (-1, -1);
    let mut b = (-1, -1);
    let mut prize = (-1, -1);
    for line in read_to_string(file_path).unwrap().lines() {
        if line.contains("Button A") {
            let (_, right) = line.split_once(':').unwrap();
            let (x, y) = right.split_once(',').unwrap();
            let x = x.trim().split('+').collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
            let y = y.trim().split('+').collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
            a = (y, x);
        } else if line.contains("Button B") {
            let (_, right) = line.split_once(':').unwrap();
            let (x, y) = right.split_once(',').unwrap();
            let x = x.trim().split('+').collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
            let y = y.trim().split('+').collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
            b = (y, x);
        } else if line.contains("Prize") {
            let (_, right) = line.split_once(':').unwrap();
            let (x, y) = right.split_once(',').unwrap();
            let x = x.trim().split('=').collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
            let y = y.trim().split('=').collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
            prize = (y, x);
        }

        if a != (-1, 1) && b != (-1, -1) && prize != (-1, -1) {
            games.push(Game { a, b, prize });
            a = (-1, -1);
            b = (-1, -1);
            prize = (-1, -1);
        }
    }

    println!("Games: {:?}", games);

    let mut count = 0;
    for game in games.iter() {
        count = count + traverse_branches(game);
        println!("Count: {} for game: {:?}", count, game);
    }

    count as usize
}

fn traverse_branches(game: &Game) -> i32 {
    let mut min = 1000;

    for i in 0..100 {
        for j in 0..100 {
            if game.a.1 * i + game.b.1 * j == game.prize.1
                && game.a.0 * i + game.b.0 * j == game.prize.0
            {
                min = cmp::min(min, i * 3 + j);
            }
        }
    }

    if min == 1000 {
        return 0;
    }

    min
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
            480
        );
    }
}
