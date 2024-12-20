use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fs::read_to_string,
    usize,
};

use itertools::Itertools;

const NUM: &str = "20";

fn main() {
    let file_path = format!("/home/phobos/code/advent_2024/src/bin/{NUM}/data.txt");
    let result = part(file_path.as_str());
    let mut answer = 0;

    for (k, v) in result.iter().sorted_by_key(|(k, _)| *k) {
        println!("{:?} {:?}", k, v);
        if k >= &100 {
            answer += v;
        }
    }

    println!("Saved seconds {:?}", answer);
}

fn part(file_path: &str) -> HashMap<usize, usize> {
    let mut grid = Vec::new();
    for line in read_to_string(file_path).unwrap().lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }

    let (start_y, start_x) = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .find(|(i, j)| grid[*i][*j] == 'S')
        .unwrap();

    let mut djikstra_min_path = dijkstras_walk(&grid, start_y, start_x);
    djikstra_min_path.insert(0, (start_y, start_x));
    let cheated_paths = find_cheating_paths(&djikstra_min_path, &grid, 20);

    let mut bucket_cheats = HashMap::new();
    for cheated_path in cheated_paths.iter() {
        let expected_cheat_duration = djikstra_min_path.len();
        let actual_cheat_duration = cheated_path;

        *bucket_cheats
            .entry(expected_cheat_duration - actual_cheat_duration)
            .or_insert(0) += 1;
    }

    bucket_cheats
}

fn dijkstras_walk(grid: &Vec<Vec<char>>, start_y: usize, start_x: usize) -> Vec<(usize, usize)> {
    let mut visited = vec![vec![false; grid.len()]; grid.len()];
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start_y, start_x, vec![])));

    while let Some(Reverse((score, y, x, moves))) = heap.pop() {
        if visited[y][x] {
            continue;
        }

        if grid[y][x] == 'E' {
            return moves;
        }

        visited[y][x] = true;

        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let ny = y as i32 + dy;
            let nx = x as i32 + dx;
            let nx = nx as usize;
            let ny = ny as usize;

            if ny >= grid.len() || nx >= grid[0].len() || grid[ny][nx] == '#' {
                continue;
            }

            let mut new_moves = moves.clone();
            new_moves.push((ny, nx));
            heap.push(Reverse((score + 1, ny, nx, new_moves)));
        }
    }

    vec![]
}

fn find_cheating_paths(
    path: &Vec<(usize, usize)>,
    grid: &Vec<Vec<char>>,
    distance: usize,
) -> Vec<usize> {
    let mut new_paths = vec![];
    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();

    for i in 0..path.len() {
        println!("{i} of {}", path.len());
        let (y, x) = path[i];

        let mut coords_within_pico_distance = HashSet::new();
        for j in i + 1..path.len() {
            let (ny, nx) = path[j];
            let manhattan_distance_y = ny as i32 - y as i32;
            let manhattan_distance_x = nx as i32 - x as i32;
            let total_distance = manhattan_distance_y.abs() + manhattan_distance_x.abs();
            if total_distance > 0 && total_distance <= distance as i32 {
                coords_within_pico_distance.insert((ny, nx, total_distance as usize));
            }
        }

        for coord in coords_within_pico_distance.iter() {
            let (ny2, nx2, total_distance) = coord;

            let mut new_path = 0;
            new_path = new_path + i;
            new_path = new_path + total_distance;

            if memo.contains_key(&(*ny2, *nx2)) {
                new_path = new_path + memo.get(&(*ny2, *nx2)).unwrap();
            } else {
                let index = path
                    .iter()
                    .position(|(y, x)| *y == *ny2 && *x == *nx2)
                    .unwrap();

                new_path = new_path + (path.len() - index);
                memo.insert((*ny2, *nx2), path.len() - index);
            }

            new_paths.push(new_path);
        }
    }

    new_paths
}

fn print_grid(
    grid: &Vec<Vec<char>>,
    path: &Vec<(usize, usize)>,
    y1: usize,
    x1: usize,
    ny2: usize,
    nx2: usize,
) {
    let mut grid = grid.clone();
    for (y, x) in path.iter() {
        if grid[*y][*x] == 'S' || grid[*y][*x] == 'E' {
            continue;
        }

        grid[*y][*x] = 'x';
        if y1 == *y && x1 == *x {
            grid[*y][*x] = '1';
        }
        if ny2 == *y && nx2 == *x {
            grid[*y][*x] = '2';
        }
    }

    for row in grid.iter() {
        println!("{}", row.iter().collect::<String>());
    }

    println!();
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    #[test]
    fn example() {
        let result = super::part(
            format!(
                "/home/phobos/code/advent_2024/src/bin/{}/data_test.txt",
                super::NUM
            )
            .as_str(),
        );

        for (key, value) in result.iter().sorted() {
            println!("Picoseconds saved: {:?}, times: {:?}", key, value);
        }

        assert_eq!(result.get(&50).unwrap(), &32);
        assert_eq!(result.get(&52).unwrap(), &31);
        assert_eq!(result.get(&54).unwrap(), &29);
        assert_eq!(result.get(&56).unwrap(), &39);
        assert_eq!(result.get(&58).unwrap(), &25);
        assert_eq!(result.get(&60).unwrap(), &23);
        assert_eq!(result.get(&62).unwrap(), &20);
        assert_eq!(result.get(&64).unwrap(), &19);
        assert_eq!(result.get(&66).unwrap(), &12);
        assert_eq!(result.get(&68).unwrap(), &14);
        assert_eq!(result.get(&70).unwrap(), &12);
        assert_eq!(result.get(&72).unwrap(), &22);
        assert_eq!(result.get(&74).unwrap(), &4);
        assert_eq!(result.get(&76).unwrap(), &3);
    }
}
