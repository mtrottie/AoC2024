use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

const NUM: &str = "15";

fn main() {
    let file_path = format!("/home/phobos/code/advent_2024/src/bin/{NUM}/data.txt");
    println!("{}", part(file_path.as_str()));
}

fn part(file_path: &str) -> usize {
    let input = read_to_string(file_path).unwrap();
    let (a, moves) = input.split_once("\n\n").unwrap();
    let grid = a
        .lines()
        .map(|l| {
            l.bytes()
                .flat_map(|b| match b {
                    b'#' => b"##",
                    b'O' => b"[]",
                    b'.' => b"..",
                    b'@' => b"@.",
                    _ => unreachable!(),
                })
                .copied()
                .map(|b| b as char)
                .collect()
        })
        .collect();

    print_grid(&grid);
    //let mut stdout = stdout();
    //print_grid_by_line(&grid, &Direction::Right(0), &mut stdout);
    //stdout.execute(cursor::Show).unwrap();
    solve(grid, moves)
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid.iter() {
        println!("{:?}", row);
    }
}

// Referenced from here: https://github.com/AxlLind/AdventOfCode2024/blob/main/src/bin/15.rs, kudos.
fn solve(mut g: Vec<Vec<char>>, insts: &str) -> usize {
    let (mut r, mut c) = (0..g.len())
        .cartesian_product(0..g[0].len())
        .find(|&(r, c)| g[r][c] == '@')
        .unwrap();

    'outer: for i in insts.bytes() {
        let (dr, dc) = match i {
            b'^' => (-1, 0),
            b'>' => (0, 1),
            b'v' => (1, 0),
            b'<' => (0, -1),
            _ => continue,
        };

        let mut q = VecDeque::from([(r, c)]);
        let mut seen = HashSet::new();
        while let Some((rr, cc)) = q.pop_front() {
            if !seen.insert((rr, cc)) {
                continue;
            }
            let (r2, c2) = (rr + dr as usize, cc + dc as usize);
            match g[r2][c2] {
                '#' => continue 'outer,
                'O' => q.push_back((r2, c2)),
                '[' => q.extend([(r2, c2), (r2, c2 + 1)]),
                ']' => q.extend([(r2, c2), (r2, c2 - 1)]),
                _ => continue,
            }
        }

        let boxes = seen
            .iter()
            .sorted_by_key(|&&(rr, cc)| (c.abs_diff(cc), r.abs_diff(rr)))
            .rev();

        for &(rr, cc) in boxes {
            let (r2, c2) = (rr + dr as usize, cc + dc as usize);
            g[r2][c2] = g[rr][cc];
            g[rr][cc] = '.';
        }

        (r, c) = (r + dr as usize, c + dc as usize);
    }

    (0..g.len())
        .cartesian_product(0..g[0].len())
        .filter(|&(r, c)| matches!(g[r][c], 'O' | '['))
        .map(|(r, c)| r * 100 + c)
        .sum()
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
            9021
        );
    }
}
