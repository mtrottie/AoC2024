use std::fs::read_to_string;

const NUM: &str = "4";

fn main() {
    let file_path = format!("/home/phobos/code/advent_2024/src/bin/{NUM}/data.txt");
    println!("{}", part(file_path.as_str()));
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

fn part(file_path: &str) -> usize {
    let mut search = Vec::new();
    for line in read_to_string(file_path).unwrap().lines() {
        let row: Vec<char> = line.chars().collect::<Vec<char>>();
        search.push(row);
    }

    let mut count = 0;
    for i in 0..search.len() {
        for j in 0..search[0].len() {
            if search[i][j] == 'X' {
                count = count + dfs(&mut search, i, j, String::new(), Direction::Down);
                count = count + dfs(&mut search, i, j, String::new(), Direction::Up);
                count = count + dfs(&mut search, i, j, String::new(), Direction::UpLeft);
                count = count + dfs(&mut search, i, j, String::new(), Direction::UpRight);
                count = count + dfs(&mut search, i, j, String::new(), Direction::DownLeft);
                count = count + dfs(&mut search, i, j, String::new(), Direction::DownRight);
                count = count + dfs(&mut search, i, j, String::new(), Direction::Left);
                count = count + dfs(&mut search, i, j, String::new(), Direction::Right);
            }
        }
    }

    println!("{:?}", count);
    count
}

fn dfs(
    search: &mut Vec<Vec<char>>,
    i: usize,
    j: usize,
    current: String,
    direction: Direction,
) -> usize {
    if i >= search.len() || j >= search[0].len() {
        return 0;
    }

    if search[i][j] == 'a' {
        return 0;
    }

    if current.len() > 4 {
        return 0;
    }

    if current.clone() + &search[i][j].to_string() == "XMAS" {
        return 1;
    }

    let char = &search[i][j].to_string();
    match direction {
        Direction::Up => {
            if i != 0 {
                dfs(search, i - 1, j, current + char, direction)
            } else {
                0
            }
        }
        Direction::Down => dfs(search, i + 1, j, current + char, direction),
        Direction::Left => {
            if j != 0 {
                dfs(search, i, j - 1, current + char, direction)
            } else {
                0
            }
        }
        Direction::Right => dfs(search, i, j + 1, current + char, direction),
        Direction::UpLeft => {
            if i != 0 && j != 0 {
                dfs(search, i - 1, j - 1, current + char, direction)
            } else {
                0
            }
        }
        Direction::UpRight => {
            if i != 0 {
                dfs(search, i - 1, j + 1, current + char, direction)
            } else {
                0
            }
        }
        Direction::DownLeft => {
            if j != 0 {
                dfs(search, i + 1, j - 1, current + char, direction)
            } else {
                0
            }
        }
        Direction::DownRight => dfs(search, i + 1, j + 1, current + char, direction),
    }
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
            18
        );
    }
}
