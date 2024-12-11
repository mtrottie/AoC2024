use std::{collections::VecDeque, fs::read_to_string};

const NUM: &str = "11";

fn main() {
    let file_path = format!("/home/phobos/code/advent_2024/src/bin/{NUM}/data.txt");
    println!("{}", part(file_path.as_str()));
}

fn part(file_path: &str) -> usize {
    let mut grid = VecDeque::new();
    for line in read_to_string(file_path).unwrap().lines() {
        grid = line
            .split(' ')
            .map(|i| i.parse::<usize>().unwrap())
            .collect::<VecDeque<usize>>();
    }

    let mut blinks = 25;

    while blinks > 0 {
        let mut new_grid_counter = 0;
        let mut new_grid = VecDeque::new();
        for j in 0..grid.len() {
            if grid[j] == 0 {
                new_grid.insert(new_grid_counter, 1);
            } else if grid[j].to_string().len() % 2 == 0 {
                let string = grid[j].to_string();
                new_grid.insert(
                    new_grid_counter,
                    string[0..string.len() / 2].parse::<usize>().unwrap(),
                );
                new_grid_counter = new_grid_counter + 1;
                new_grid.insert(
                    new_grid_counter,
                    string[string.len() / 2..string.len()]
                        .parse::<usize>()
                        .unwrap(),
                );
            } else {
                new_grid.insert(new_grid_counter, grid[j] * 2024);
            }

            new_grid_counter = new_grid_counter + 1;
        }

        grid = new_grid;
        blinks = blinks - 1;
    }

    grid.len()
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
            55312
        );
    }
}
