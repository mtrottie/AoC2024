use std::fs::read_to_string;

const NUM: &str = "4";

fn main() {
    let file_path = format!("/home/phobos/code/advent_2024/src/bin/{NUM}/data.txt");
    println!("{}", part(file_path.as_str()));
}

fn part(file_path: &str) -> usize {
    let mut search = Vec::new();
    for line in read_to_string(file_path).unwrap().lines() {
        let row: Vec<char> = line.chars().collect::<Vec<char>>();
        search.push(row);
    }

    let mut count = 0;
    for i in 1..search.len() - 1 {
        for j in 1..search[0].len() - 1 {
            if search[i][j] == 'A' {
                count = count + dfs(&mut search, i, j);
            }
        }
    }

    println!("{:?}", count);
    count
}

fn dfs(search: &mut Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let mut up_left_to_down_right = false;
    if (search[i - 1][j - 1] == 'S' && search[i + 1][j + 1] == 'M')
        || (search[i - 1][j - 1] == 'M' && search[i + 1][j + 1] == 'S')
    {
        up_left_to_down_right = true;
    }

    let mut up_right_to_down_left = false;
    if (search[i - 1][j + 1] == 'S' && search[i + 1][j - 1] == 'M')
        || (search[i - 1][j + 1] == 'M' && search[i + 1][j - 1] == 'S')
    {
        up_right_to_down_left = true;
    }

    if up_left_to_down_right && up_right_to_down_left {
        return 1;
    }

    0
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
            9
        );
    }
}
