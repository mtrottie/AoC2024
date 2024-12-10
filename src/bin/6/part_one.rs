use std::fs::read_to_string;

const NUM: &str = "6";

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let file_path = format!("/home/phobos/code/advent_2024/src/bin/{NUM}/data.txt");
    println!("{}", part(file_path.as_str()));
}

fn part(file_path: &str) -> usize {
    let mut search = Vec::new();
    let mut visited = Vec::new();
    for line in read_to_string(file_path).unwrap().lines() {
        let row: Vec<char> = line.chars().collect::<Vec<char>>();
        search.push(row.clone());
        visited.push(row);
    }

    println!("{:?}", search);

    for i in 0..search.len() {
        for j in 0..search[i].len() {
            if search[i][j] == '^' {
                let visited = follow_the_path(&search, Direction::Up, j, i, &mut visited);
                println!("{:?}", visited);
            }
        }
    }

    let mut count = 0;
    for row in visited {
        for col in row {
            if col == 'X' {
                count += 1;
            }
        }
    }

    count
}

fn follow_the_path(
    search: &Vec<Vec<char>>,
    direction: Direction,
    x: usize,
    y: usize,
    visited: &mut Vec<Vec<char>>,
) -> Vec<Vec<char>> {
    println!("{:?} {} {}", direction, x, y);
    visited[y][x] = 'X';

    match direction {
        Direction::Up => {
            if y == 0 {
                return visited.to_vec();
            }

            if search[y - 1][x] == '#' {
                return follow_the_path(search, Direction::Right, x + 1, y, visited);
            }

            follow_the_path(search, direction, x, y - 1, visited)
        }
        Direction::Down => {
            if y + 1 >= search.len() {
                return visited.to_vec();
            }

            if search[y + 1][x] == '#' {
                if x == 0 {
                    return visited.to_vec();
                }

                return follow_the_path(search, Direction::Left, x - 1, y, visited);
            }

            follow_the_path(search, direction, x, y + 1, visited)
        }
        Direction::Left => {
            if x == 0 {
                return visited.to_vec();
            }

            if search[y][x - 1] == '#' {
                if y == 0 {
                    return visited.to_vec();
                }

                return follow_the_path(search, Direction::Up, x, y - 1, visited);
            }

            follow_the_path(search, direction, x - 1, y, visited)
        }
        Direction::Right => {
            if x + 1 >= search[y].len() {
                return visited.to_vec();
            }

            if search[y][x + 1] == '#' {
                return follow_the_path(search, Direction::Down, x, y + 1, visited);
            }

            follow_the_path(search, direction, x + 1, y, visited)
        }
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
            41
        );
    }
}
