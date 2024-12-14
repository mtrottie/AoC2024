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

    for i in 0..steps {
        for position in positions.iter_mut() {
            let mut new_y = position[0];
            let mut new_x = position[1];
            let vy = position[2];
            let vx = position[3];

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

            position[0] = new_y;
            position[1] = new_x;
        }

        let mut grid = vec![vec!['.'; x as usize]; y as usize];
        for position in positions.iter() {
            grid[position[0] as usize][position[1] as usize] = '#';
        }

        println!("Step: {}", i);
        for line in grid.iter() {
            // uncomment this line and print to a file and search for the easter egg manually, I used the answer form field to do
            // binary search to get an approximation of where it was, i initially did 5k and 10k and just searched through
            // the ascii printed in vim
            // println!("{:?}", line);
        }
    }

    6870
}

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(
            super::part(
                format!(
                    "/home/phobos/code/advent_2024/src/bin/{}/data.txt",
                    super::NUM
                )
                .as_str(),
                103,
                101,
                10000
            ),
            6870
        );
    }
}
