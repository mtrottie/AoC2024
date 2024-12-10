use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use itertools::Itertools;

const NUM: &str = "8";

fn main() {
    let file_path = format!("/home/phobos/code/advent_2024/src/bin/{NUM}/data.txt");
    println!("{}", part(file_path.as_str()));
}

fn part(file_path: &str) -> usize {
    let mut grid = Vec::new();
    for line in read_to_string(file_path).unwrap().lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }

    println!("Grid: {:?}", grid);
    let mut coords: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != '.' {
                coords.entry(grid[i][j]).or_default().push((i, j));
            }
        }
    }

    println!("Coords: {:?}", coords);
    let anti_nodes = count_unique_anti_nodes(&coords, grid.len(), grid[0].len());
    for node in anti_nodes.iter() {
        grid[node.0][node.1] = '#';
    }

    for row in grid.iter() {
        println!("{}", row.iter().join(""));
    }
    println!("Antinodes: {:?}", anti_nodes);

    anti_nodes.len()
}

fn count_unique_anti_nodes(
    coords: &HashMap<char, Vec<(usize, usize)>>,
    y: usize,
    x: usize,
) -> HashSet<(usize, usize)> {
    let mut seen = HashSet::new();

    for (key, value) in coords.into_iter() {
        println!("Iterating through key: {key} with values: {:?}", value);

        for i in 0..value.len() {
            for j in 0..value.len() {
                if i == j {
                    continue;
                }

                let y_manhattan_distance = value[i].0 - value[j].0;
                let x_manhattan_distance = value[i].1 - value[j].1;

                let anti_node_1 = (
                    value[i].0 + y_manhattan_distance,
                    value[i].1 + x_manhattan_distance,
                );
                let anti_node_2 = (
                    value[j].0 - y_manhattan_distance,
                    value[j].1 - x_manhattan_distance,
                );

                if anti_node_1.0 < y && anti_node_1.1 < x {
                    seen.insert(anti_node_1);
                }
                if anti_node_2.0 < y && anti_node_2.1 < x {
                    seen.insert(anti_node_2);
                }
            }
        }
    }

    seen
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
            14
        );
    }
}
