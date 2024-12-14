use std::fs::read_to_string;

const NUM: &str = "14";

fn main() {
    let file_path = format!("/home/phobos/code/advent_2024/src/bin/{NUM}/data.txt");
    println!("{}", part(file_path.as_str(), 103, 101, 100));
}

fn part(file_path: &str, y: i32, x: i32, steps: i32) -> usize {
    let mut positions = Vec::new();
    for line in read_to_string(file_path).unwrap().lines() {
        let (p, v) = line.split_once(' ').unwrap();
        let (x, y) = p.split('=').last().unwrap().split_once(',').unwrap();
        let (vx, vy) = v.split('=').last().unwrap().split_once(',').unwrap();
        positions.push(vec![
            y.parse::<i32>().unwrap(),
            x.parse::<i32>().unwrap(),
            vy.parse::<i32>().unwrap(),
            vx.parse::<i32>().unwrap(),
        ]);
    }

    println!("Positions: {:?}", positions);

    let mut quadrants = [0, 0, 0, 0];
    let mut final_positions = Vec::new();
    for position in positions {
        let mut new_y = position[0];
        let mut new_x = position[1];
        let vy = position[2];
        let vx = position[3];

        println!(
            "Using coordinates: y: {:?} x:{:?}, vy:{:?}, vx:{:?}",
            new_y, new_x, vy, vx
        );

        for _ in 0..steps {
            println!("new_y: {:?}, new_x: {:?}", new_y, new_x);
            if new_y + vy >= y {
                new_y = (new_y + vy) - y;
            } else if new_y + vy < 0 {
                new_y = y + (new_y + vy);
            } else {
                new_y = new_y + vy;
            }

            if new_x + vx >= x {
                new_x = (new_x + vx) - x;
            } else if new_x + vx < 0 {
                new_x = x + (new_x + vx);
            } else {
                new_x = new_x + vx;
            }
        }

        println!("new_y: {:?}, new_x: {:?}", new_y, new_x);
        final_positions.push((new_y, new_x));

        if new_y > y / 2 && new_x > x / 2 {
            quadrants[3] += 1;
        } else if new_y < y / 2 && new_x < x / 2 {
            quadrants[0] += 1;
        } else if new_y > y / 2 && new_x < x / 2 {
            quadrants[2] += 1;
        } else if new_y < y / 2 && new_x > x / 2 {
            quadrants[1] += 1;
        }
    }

    println!("Final Positions: {:?}", final_positions);
    println!("Quadrants: {:?}", quadrants);

    let mut grid = vec![vec!['.'; x as usize]; y as usize];
    for position in final_positions {
        grid[position.0 as usize][position.1 as usize] = '#';
    }

    for line in grid.iter() {
        println!("{:?}", line);
    }

    quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
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
                11,
                100
            ),
            12
        );
    }
}
