use std::fs::read_to_string;

const NUM: &str = "13";

fn main() {
    let file_path = format!("/home/phobos/code/advent_2024/src/bin/{NUM}/data.txt");
    println!("{}", part(file_path.as_str()));
}

#[derive(Debug)]
struct Game {
    a: (i128, i128),
    b: (i128, i128),
    prize: (i128, i128),
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
                .parse::<i128>()
                .unwrap();
            let y = y.trim().split('+').collect::<Vec<&str>>()[1]
                .parse::<i128>()
                .unwrap();
            a = (y, x);
        } else if line.contains("Button B") {
            let (_, right) = line.split_once(':').unwrap();
            let (x, y) = right.split_once(',').unwrap();
            let x = x.trim().split('+').collect::<Vec<&str>>()[1]
                .parse::<i128>()
                .unwrap();
            let y = y.trim().split('+').collect::<Vec<&str>>()[1]
                .parse::<i128>()
                .unwrap();
            b = (y, x);
        } else if line.contains("Prize") {
            let (_, right) = line.split_once(':').unwrap();
            let (x, y) = right.split_once(',').unwrap();
            let x = x.trim().split('=').collect::<Vec<&str>>()[1]
                .parse::<i128>()
                .unwrap();
            let y = y.trim().split('=').collect::<Vec<&str>>()[1]
                .parse::<i128>()
                .unwrap();
            prize = (y + 10000000000000, x + 10000000000000);
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

// a*x_a + b*x_b = p_x
// a*y_a + b*y_b = p_y
// b = (tx*ay-ty*ax)//(ay*bx-by*ax)
// a = (tx*by-ty*bx)//(by*ax-bx*ay)
fn traverse_branches(game: &Game) -> i128 {
    let determinant = game.a.0 * game.b.1 - game.a.1 * game.b.0;

    if determinant == 0 {
        return 0;
    }

    let x = (game.prize.0 * game.b.1 - game.prize.1 * game.b.0) as f64 / determinant as f64;
    let y = (game.a.0 * game.prize.1 - game.a.1 * game.prize.0) as f64 / determinant as f64;

    if x.trunc() != x || y.trunc() != y {
        return 0;
    }

    x as i128 * 3 + y as i128
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
            875318608908
        );
    }
}
