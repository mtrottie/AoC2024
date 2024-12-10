use itertools::Itertools;
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
    println!("{:?}", part(file_path.as_str()));
}

fn part(file_path: &str) -> (usize, usize) {
    let mut search = read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let (starting_y, starting_x) = (0..search.len())
        .cartesian_product(0..search[0].len())
        .find(|&(r, c)| search[r][c] == b'^')
        .unwrap();

    let walked = walk(&search, starting_y, starting_x, true).unwrap();
    println!("Walked length: {}", walked.len());

    let count = walked
        .iter()
        .filter(|&&(y, x)| {
            search[y][x] = b'#';
            let ok = walk(&search, starting_y, starting_x, false).is_none();
            search[y][x] = b'.';
            ok
        })
        .count();

    (walked.len(), count)
}

fn walk(
    search: &[Vec<u8>],
    mut starting_y: usize,
    mut starting_x: usize,
    return_squares: bool,
) -> Option<Vec<(usize, usize)>> {
    let mut seen = vec![vec![[false; 4]; search[0].len()]; search[0].len()];
    let mut direction = 0;

    loop {
        if seen[starting_y][starting_x][direction] {
            return None;
        }

        seen[starting_y][starting_x][direction] = true;
        let (direction_y, direction_x) = [(-1, 0), (0, 1), (1, 0), (0, -1)][direction];

        let (new_y, new_x) = (
            starting_y + direction_y as usize,
            starting_x + direction_x as usize,
        );

        if !(0..search.len()).contains(&new_y) || !(0..search[0].len()).contains(&new_x) {
            if !return_squares {
                return Some(Vec::new());
            }

            let visited = (0..search.len())
                .cartesian_product(0..search[0].len())
                .filter(|&(r, c)| seen[r][c].iter().any(|&b| b))
                .collect();

            return Some(visited);
        }

        if search[new_y][new_x] == b'#' {
            direction = (direction + 1) % 4;
        } else {
            (starting_y, starting_x) = (new_y, new_x)
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
            (41, 6)
        );
    }
}
