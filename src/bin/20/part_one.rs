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
    let mut paths = &mut vec![];
    find_paths(&mut grid, start_y, start_x, vec![], paths);
    paths[0].insert(0, (start_y, start_x));

    assert_eq!(paths[0], djikstra_min_path);

    println!("Found legitimate paths: {:?}", paths.len());
    println!(
        "Legitimate path length djikstra: {:?}",
        djikstra_min_path.len()
    );
    println!("Legitimate path length dfs: {:?}", paths[0].len());
    println!("Path: {:?}", djikstra_min_path);

    for path in paths.iter() {
        let cheated_paths = find_cheating_paths(path, &grid);

        let mut bucket_cheats = HashMap::new();
        for cheated_path in cheated_paths.iter() {
            let expected_cheat_duration = path.len();
            let actual_cheat_duration = cheated_path.len();

            *bucket_cheats
                .entry(expected_cheat_duration - actual_cheat_duration)
                .or_insert(0) += 1;
        }

        return bucket_cheats;
    }

    HashMap::new()
}

fn find_paths(
    grid: &mut Vec<Vec<char>>,
    y: usize,
    x: usize,
    path: Vec<(usize, usize)>,
    paths: &mut Vec<Vec<(usize, usize)>>,
) {
    if y > grid.len() - 1 || x > grid[0].len() - 1 || grid[y][x] == '-' || grid[y][x] == '#' {
        return;
    }

    let mut path = path.clone();
    if grid[y][x] != 'S' {
        path.push((y, x));
    }

    if grid[y][x] == 'E' {
        paths.push(path.clone());
        return;
    }

    let temp = grid[y][x];
    grid[y][x] = '-';
    find_paths(grid, y, x - 1, path.clone(), paths);
    find_paths(grid, y, x + 1, path.clone(), paths);
    find_paths(grid, y - 1, x, path.clone(), paths);
    find_paths(grid, y + 1, x, path.clone(), paths);
    grid[y][x] = temp;
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

// Right, Left, Up, Down
const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn find_cheating_paths(
    path: &Vec<(usize, usize)>,
    grid: &Vec<Vec<char>>,
) -> Vec<Vec<(usize, usize)>> {
    let mut new_paths = vec![];

    for i in 0..path.len() {
        let (y, x) = path[i];

        for dir in DIRECTIONS {
            let (dy, dx) = dir;
            let (ny, nx) = (y as i32 + dy, x as i32 + dx);

            for second_turn in DIRECTIONS {
                let (dy2, dx2) = second_turn;
                let (ny2, nx2) = (ny as i32 + dy2, nx as i32 + dx2);

                let sub_path = &path[i + 1..];
                if sub_path.contains(&(ny2 as usize, nx2 as usize)) {
                    let mut new_path = vec![];
                    let index = path
                        .iter()
                        .position(|(y, x)| *y == ny2 as usize && *x == nx2 as usize)
                        .unwrap();

                    new_path.extend_from_slice(&path[0..i + 1]);
                    new_path.push((ny as usize, nx as usize));
                    new_path.extend_from_slice(&path[index..]);
                    new_paths.push(new_path.clone());
                }
            }
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

        assert_eq!(result.get(&2).unwrap(), &14);
        assert_eq!(result.get(&4).unwrap(), &14);
        assert_eq!(result.get(&6).unwrap(), &2);
        assert_eq!(result.get(&8).unwrap(), &4);
        assert_eq!(result.get(&10).unwrap(), &2);
        assert_eq!(result.get(&12).unwrap(), &3);
        assert_eq!(result.get(&20).unwrap(), &1);
        assert_eq!(result.get(&36).unwrap(), &1);
        assert_eq!(result.get(&38).unwrap(), &1);
        assert_eq!(result.get(&40).unwrap(), &1);
        assert_eq!(result.get(&64).unwrap(), &1);
    }
}
