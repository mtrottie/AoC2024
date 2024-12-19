use std::{cmp::Reverse, collections::BinaryHeap, fs::read_to_string};

const NUM: &str = "18";

fn main() {
    let file_path = format!("/home/phobos/code/advent_2024/src/bin/{NUM}/data.txt");
    println!("{}", part(file_path.as_str(), 71, 1024));
}

fn part(file_path: &str, size: usize, steps: usize) -> usize {
    let mut grid = vec![vec!['.'; size]; size];

    let mut fallen = 0;
    for line in read_to_string(file_path).unwrap().lines() {
        if steps == fallen {
            break;
        }
        let (x, y) = line.split_once(",").unwrap();
        grid[y.parse::<usize>().unwrap()][x.parse::<usize>().unwrap()] = '#';
        fallen += 1;
    }

    print_grid(&grid);
    
    dijkstras_walk(&grid, size - 1, size - 1)
}

fn dijkstras_walk(grid: &Vec<Vec<char>>, target_y: usize, target_x: usize) -> usize {
    let mut visited = vec![vec![false; grid.len()]; grid.len()];
    let mut count = 0;

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, 0, 0)));

    while let Some(Reverse((score, y, x))) = heap.pop() {
        if visited[y][x] {
            continue;
        }

        if y == target_y && x == target_x {
            count = score;
            break;
        }

        visited[y][x] = true;

        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let ny = y as i32 + dy;
            let nx = x as i32 + dx;

            if nx < 0 || nx >= grid[0].len() as i32 || ny < 0 || ny >= grid.len() as i32 {
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            if grid[ny][nx] == '.' {
                heap.push(Reverse((score + 1, ny, nx)));
            }
        }
    }

    count
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{:?}", row);
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
                .as_str(),
                7,
                12
            ),
            22
        );
    }
}
