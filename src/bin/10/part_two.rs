use std::fs::read_to_string;

const NUM: &str = "10";

fn main() {
    let file_path = format!("/home/phobos/code/advent_2024/src/bin/{NUM}/data.txt");
    println!("{}", part(file_path.as_str()));
}

fn part(file_path: &str) -> usize {
    let mut grid = Vec::new();
    for line in read_to_string(file_path).unwrap().lines() {
        grid.push(
            line.chars()
                .map(|i| i.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>(),
        );
    }

    for i in 0..grid.len() {
        println!("{:?}", grid[i]);
    }

    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                let trail_heads = dfs(&mut grid, i, j, 0);
                println!("Number of trailheads ({i}, {j}): {trail_heads}");
                count = count + trail_heads;
            }
        }
    }

    count
}

fn dfs(grid: &mut Vec<Vec<usize>>, i: usize, j: usize, height: usize) -> usize {
    if i >= grid.len() || j >= grid[0].len() || grid[i][j] != height {
        return 0;
    }

    if grid[i][j] == 9 {
        // println!("Successful Grid.");
        // for i in 0..grid.len() {
        //    println!("{:?}", grid[i]);
        // }

        return 1;
    }

    let temp = grid[i][j];
    grid[i][j] = height + 10;

    let count = dfs(grid, i + 1, j, height + 1)
        + dfs(grid, i - 1, j, height + 1)
        + dfs(grid, i, j + 1, height + 1)
        + dfs(grid, i, j - 1, height + 1);

    grid[i][j] = temp;

    count
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
            81
        );
    }
}
