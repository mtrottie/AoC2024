use std::{collections::HashSet, fs::read_to_string};

const NUM: &str = "12";

fn main() {
    let file_path = format!("/home/phobos/code/advent_2024/src/bin/{NUM}/data.txt");
    println!("{}", part(file_path.as_str()));
}

fn part(file_path: &str) -> usize {
    let mut grid = Vec::new();
    for line in read_to_string(file_path).unwrap().lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }

    let mut islands = Vec::new();
    let mut visited = HashSet::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if !visited.contains(&(y, x)) {
                let seed = grid[y][x];
                islands.push(walk_island(&mut grid, x, y, seed, &mut visited));
            }
        }
    }

    for row in grid.iter() {
        println!("{:?}", row);
    }

    let mut answer = 0;
    for island in islands {
        let area = island.len();
        let set = island
            .iter()
            .map(|n| (n.y as i32, n.x as i32))
            .collect::<HashSet<_>>();
        let sides = count_region_sides(&set);

        println!("Area: {:?} sides: {:?}", area, sides);
        answer = answer + area * sides;
    }

    answer
}

const DIR: [(i32, i32); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];

fn count_region_sides(region: &HashSet<(i32, i32)>) -> usize {
    let mut side_count = 0;

    for dir in DIR {
        let mut sides: HashSet<(i32, i32)> = HashSet::default();

        for pos in region {
            let tmp = (pos.0 + dir.0, pos.1 + dir.1);

            if !region.contains(&tmp) {
                sides.insert(tmp);
            }
        }

        let mut remove: HashSet<(i32, i32)> = HashSet::default();
        for side in &sides {
            let mut tmp = (side.0 + dir.1, side.1 + dir.0);

            while sides.contains(&tmp) {
                remove.insert(tmp);
                tmp = (tmp.0 + dir.1, tmp.1 + dir.0);
            }
        }

        println!("Remove: {:?}", remove);
        side_count += sides.len() - remove.len();
    }

    side_count
}

#[derive(Debug, PartialEq, Eq)]
pub enum Perimeter {
    Fence,
    NoFence,
}

#[derive(Debug)]
pub struct Node {
    x: usize,
    y: usize,
    value: char,
    left: Perimeter,
    right: Perimeter,
    up: Perimeter,
    down: Perimeter,
}

fn walk_island(
    grid: &mut Vec<Vec<char>>,
    x: usize,
    y: usize,
    seed: char,
    visited: &mut HashSet<(usize, usize)>,
) -> Vec<Node> {
    if y > grid.len() || x > grid[y].len() || visited.contains(&(y, x)) {
        return vec![];
    }

    visited.insert((y, x));
    let mut nodes = Vec::new();

    let left;
    if x == 0 {
        left = Perimeter::Fence;
    } else if grid[y][x - 1] == seed {
        left = Perimeter::NoFence;
        nodes.append(&mut walk_island(grid, x - 1, y, seed, visited));
    } else {
        left = Perimeter::Fence;
    }

    let right;
    if x == grid[0].len() - 1 {
        right = Perimeter::Fence;
    } else if grid[y][x + 1] == seed {
        right = Perimeter::NoFence;
        nodes.append(&mut walk_island(grid, x + 1, y, seed, visited));
    } else {
        right = Perimeter::Fence;
    }

    let up;
    if y == 0 {
        up = Perimeter::Fence;
    } else if grid[y - 1][x] == seed {
        up = Perimeter::NoFence;
        nodes.append(&mut walk_island(grid, x, y - 1, seed, visited));
    } else {
        up = Perimeter::Fence;
    }

    let down;
    if y == grid.len() - 1 {
        down = Perimeter::Fence;
    } else if grid[y + 1][x] == seed {
        down = Perimeter::NoFence;
        nodes.append(&mut walk_island(grid, x, y + 1, seed, visited));
    } else {
        down = Perimeter::Fence;
    }

    nodes.push(Node {
        x,
        y,
        value: seed,
        left,
        right,
        up,
        down,
    });

    nodes
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
            1206
        );
    }
}
