use std::{
    collections::VecDeque,
    fs::read_to_string,
    io::{stdout, Stdout, Write},
    thread, time,
};

use crossterm::{
    cursor,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal, ExecutableCommand, QueueableCommand,
};

const NUM: &str = "15";

#[derive(Debug)]
pub enum Direction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

fn main() {
    let file_path = format!("/home/phobos/code/advent_2024/src/bin/{NUM}/data.txt");
    println!("{}", part(file_path.as_str()));
}

fn part(file_path: &str) -> usize {
    let mut new_line_seen = false;
    let mut grid = Vec::new();
    let mut moves = Vec::new();

    for line in read_to_string(file_path).unwrap().lines() {
        if line.is_empty() {
            new_line_seen = true;
            continue;
        }

        if !new_line_seen {
            grid.push(line.chars().collect::<Vec<_>>());
        } else {
            for c in line.chars() {
                match c {
                    '^' => moves.push(Direction::Up(1)),
                    'v' => moves.push(Direction::Down(1)),
                    '<' => moves.push(Direction::Left(1)),
                    '>' => moves.push(Direction::Right(1)),
                    _ => panic!("Invalid direction"),
                }
            }
        }
    }

    let mut stdout = stdout();

    // merge_directions(&mut moves);

    for m in moves.iter() {
        let (mut starting_y, mut starting_x) = starting_positions(&grid);

        print_grid_by_line(&grid, m, &mut stdout);

        match m {
            Direction::Up(i) => {
                let mut stack = create_stack(&grid, &starting_y, &starting_x, Direction::Up(0));
                let initial_length = stack.len();
                if initial_length == 0 {
                    continue;
                }

                for j in 0..i.clone() {
                    if is_next_blockage(&grid, &(starting_y + j), &starting_x, Direction::Up(0)) {
                        break;
                    }

                    let stack_len = stack.len();
                    remove_space(&mut stack);
                    if stack.len() == stack_len {
                        break;
                    }
                }

                let mut ending_length = (initial_length - stack.len()) as i32;

                while ending_length > 0 || !stack.is_empty() {
                    if ending_length > 0 {
                        grid[starting_y][starting_x] = '.';
                        ending_length -= 1;
                    } else {
                        grid[starting_y][starting_x] = stack.pop_front().unwrap();
                    }

                    starting_y -= 1;
                }
            }
            Direction::Down(i) => {
                let mut stack = create_stack(&grid, &starting_y, &starting_x, Direction::Down(0));
                let initial_length = stack.len();

                if initial_length == 0 {
                    continue;
                }

                for j in 0..i.clone() {
                    if is_next_blockage(&grid, &starting_y, &(starting_x + j), Direction::Down(0)) {
                        break;
                    }

                    let stack_len = stack.len();
                    remove_space(&mut stack);
                    if stack.len() == stack_len {
                        break;
                    }
                }

                let mut ending_length = (initial_length - stack.len()) as i32;

                while ending_length > 0 || !stack.is_empty() {
                    if ending_length > 0 {
                        grid[starting_y][starting_x] = '.';
                        ending_length -= 1;
                    } else {
                        grid[starting_y][starting_x] = stack.pop_front().unwrap();
                    }

                    starting_y += 1;
                }
            }
            Direction::Left(i) => {
                let mut stack = create_stack(&grid, &starting_y, &starting_x, Direction::Left(0));
                let initial_length = stack.len();

                if initial_length == 0 {
                    continue;
                }

                for j in 0..i.clone() {
                    if is_next_blockage(&grid, &starting_y, &(starting_x - j), Direction::Left(0)) {
                        break;
                    }

                    let stack_len = stack.len();
                    remove_space(&mut stack);
                    if stack.len() == stack_len {
                        break;
                    }
                }

                let mut ending_length = (initial_length - stack.len()) as i32;

                while ending_length > 0 || !stack.is_empty() {
                    if ending_length > 0 {
                        grid[starting_y][starting_x] = '.';
                        ending_length -= 1;
                    } else {
                        grid[starting_y][starting_x] = stack.pop_front().unwrap();
                    }

                    starting_x -= 1;
                }
            }
            Direction::Right(i) => {
                let mut stack = create_stack(&grid, &starting_y, &starting_x, Direction::Right(0));
                let initial_length = stack.len();

                if initial_length == 0 {
                    continue;
                }

                for j in 0..i.clone() {
                    if is_next_blockage(&grid, &starting_y, &(starting_x + j), Direction::Right(0))
                    {
                        break;
                    }

                    let stack_len = stack.len();
                    remove_space(&mut stack);
                    if stack.len() == stack_len {
                        break;
                    }
                }

                let mut ending_length = (initial_length - stack.len()) as i32;

                while ending_length > 0 || !stack.is_empty() {
                    if ending_length > 0 {
                        grid[starting_y][starting_x] = '.';
                        ending_length -= 1;
                    } else {
                        grid[starting_y][starting_x] = stack.pop_front().unwrap();
                    }

                    starting_x += 1;
                }
            }
        }
    }

    stdout.execute(cursor::Show).unwrap();

    calculate_answer(&grid)
}

fn calculate_answer(grid: &Vec<Vec<char>>) -> usize {
    let mut answer = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'O' {
                answer += i * 100 + j;
            }
        }
    }

    answer
}

fn print_grid_by_line(grid: &Vec<Vec<char>>, direction: &Direction, stdout: &mut Stdout) {
    let mut text = format!("\n\n\n Direction: {:?} \n\n\n", direction);

    for row in grid.iter() {
        text = format!("{text}{:?}\n", row);
    }

    stdout.queue(cursor::SavePosition).unwrap();
    stdout.write_all(text.as_bytes()).unwrap();
    stdout.queue(cursor::RestorePosition).unwrap();
    stdout.flush().unwrap();
    thread::sleep(time::Duration::from_millis(25));

    stdout.queue(cursor::RestorePosition).unwrap();
    stdout
        .queue(terminal::Clear(terminal::ClearType::FromCursorDown))
        .unwrap();
}

fn remove_space(stack: &mut VecDeque<char>) {
    for i in 0..stack.len() {
        if stack[i] != '.' {
            continue;
        } else {
            stack.remove(i);
            break;
        }
    }
}

fn create_stack(
    grid: &Vec<Vec<char>>,
    y: &usize,
    x: &usize,
    direction: Direction,
) -> VecDeque<char> {
    let mut stack = VecDeque::new();

    let y = y.clone();
    let x = x.clone();

    match direction {
        Direction::Up(_) => {
            for i in (0..y + 1).rev() {
                if grid[i][x] == '#' {
                    break;
                }
                stack.push_back(grid[i][x]);
            }
        }
        Direction::Down(_) => {
            for i in y..grid.len() {
                if grid[i][x] == '#' {
                    break;
                }
                stack.push_back(grid[i][x]);
            }
        }
        Direction::Left(_) => {
            for i in (0..x + 1).rev() {
                if grid[y][i] == '#' {
                    break;
                }
                stack.push_back(grid[y][i]);
            }
        }
        Direction::Right(_) => {
            for i in x..grid[y].len() {
                if grid[y][i] == '#' {
                    break;
                }
                stack.push_back(grid[y][i]);
            }
        }
    }

    stack
}

fn is_next_blockage(grid: &Vec<Vec<char>>, y: &usize, x: &usize, direction: Direction) -> bool {
    let y = y.clone();
    let x = x.clone();

    let char = match direction {
        Direction::Up(_) => grid[y - 1][x],
        Direction::Down(_) => grid[y + 1][x],
        Direction::Left(_) => grid[y][x - 1],
        Direction::Right(_) => grid[y][x + 1],
    };

    char == '#'
}

fn starting_positions(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '@' {
                return (y, x);
            }
        }
    }

    panic!("No starting position found");
}

#[cfg(test)]
mod tests {
    #[test]
    fn small() {
        assert_eq!(
            super::part(
                format!(
                    "/home/phobos/code/advent_2024/src/bin/{}/data_test_small.txt",
                    super::NUM
                )
                .as_str()
            ),
            2028
        );
    }

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
            10092
        );
    }
}
