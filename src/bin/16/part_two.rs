use std::{
    cmp::{self, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    usize,
};

const NUM: &str = "16";

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn value(&self) -> (i32, i32) {
        match *self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn counter_clockwise(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn clockwise(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn main() {
    let file_path = format!("/home/phobos/code/advent_2024/src/bin/{NUM}/data.txt");
    println!("{}", part(file_path.as_str()));
}

fn part(file_path: &str) -> usize {
    let maze: Vec<Vec<char>> = std::fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let (start_y, start_x) = maze
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &c)| (i, j, c)))
        .find(|&(_, _, c)| c == 'S')
        .map(|(i, j, _)| (i, j))
        .unwrap();

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start_y, start_x, Direction::Right)));

    let mut visited = HashSet::new();

    let mut min_score = usize::MAX;
    while let Some(Reverse((score, y, x, dir))) = heap.pop() {
        if maze[y][x] == 'E' {
            min_score = score;
            break;
        }

        if !visited.insert((y, x, dir)) {
            continue;
        }

        let (dy, dx) = dir.value();
        let new_y = y as i32 + dy;
        let new_x = x as i32 + dx;
        if is_valid(new_y, new_x, &maze) {
            heap.push(Reverse((score + 1, new_y as usize, new_x as usize, dir)));
        }

        heap.push(Reverse((score + 1000, y, x, dir.clockwise())));
        heap.push(Reverse((score + 1000, y, x, dir.counter_clockwise())));
    }

    println!("min_score: {}", min_score);
    let path = &mut HashSet::new();
    dfs(
        &maze,
        start_y,
        start_x,
        Direction::Right,
        &mut HashSet::new(),
        path,
        min_score,
        0,
        &mut HashMap::new(),
    );

    path.len()
}

fn dfs(
    maze: &Vec<Vec<char>>,
    y: usize,
    x: usize,
    dir: Direction,
    visited: &mut HashSet<(usize, usize, Direction)>,
    path: &mut HashSet<(usize, usize)>,
    target: usize,
    score: usize,
    memo: &mut HashMap<(usize, usize, Direction, usize), usize>,
) -> usize {
    if let Some(&cached_result) = memo.get(&(y, x, dir, score)) {
        return cached_result;
    }

    if !is_valid(y as i32, x as i32, maze) || visited.contains(&(y, x, dir)) || score > target {
        return usize::MAX;
    }

    if maze[y][x] == 'E' && target == score {
        path.insert((y, x));
        return target;
    }

    visited.insert((y, x, dir));
    let straight = dfs(
        maze,
        (y as i32 + dir.value().0) as usize,
        (x as i32 + dir.value().1) as usize,
        dir,
        visited,
        path,
        target,
        score + 1,
        memo,
    );

    let clockwise_dir = dir.clockwise();
    let clockwise = dfs(
        maze,
        (y as i32 + clockwise_dir.value().0) as usize,
        (x as i32 + clockwise_dir.value().1) as usize,
        clockwise_dir,
        visited,
        path,
        target,
        score + 1001,
        memo,
    );

    let counter_clockwise_dir = dir.counter_clockwise();
    let counter_clockwise = dfs(
        maze,
        (y as i32 + counter_clockwise_dir.value().0) as usize,
        (x as i32 + counter_clockwise_dir.value().1) as usize,
        counter_clockwise_dir,
        visited,
        path,
        target,
        score + 1001,
        memo,
    );
    visited.remove(&(y, x, dir));

    let result = if straight == target || counter_clockwise == target || clockwise == target {
        path.insert((y, x));
        target
    } else {
        usize::MAX
    };

    memo.insert((y, x, dir, score), result);

    result
}

fn is_valid(y: i32, x: i32, maze: &Vec<Vec<char>>) -> bool {
    y >= 0
        && x >= 0
        && (y as usize) < maze.len()
        && (x as usize) < maze[0].len()
        && maze[y as usize][x as usize] != '#'
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
            45
        );
    }

    #[test]
    fn test_two() {
        assert_eq!(
            super::part(
                format!(
                    "/home/phobos/code/advent_2024/src/bin/{}/data_test_2.txt",
                    super::NUM
                )
                .as_str()
            ),
            64
        );
    }
}
