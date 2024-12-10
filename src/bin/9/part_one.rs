use std::{collections::VecDeque, fs::read_to_string};

const NUM: &str = "9";

fn main() {
    let file_path = format!("/home/phobos/code/advent_2024/src/bin/{NUM}/data.txt");
    println!("{}", part(file_path.as_str()));
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Item {
    Block(Block),
    FreeSpace,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Block {
    id: usize,
}

fn part(file_path: &str) -> usize {
    let mut queue = VecDeque::new();
    let mut id = 0;
    for line in read_to_string(file_path).unwrap().lines() {
        for i in (0..line.len()).step_by(2) {
            if i + 1 >= line.len() {
                let blocks_char = line.chars().nth(i).unwrap();
                let block = blocks_char.to_digit(10).unwrap();

                (0..block).for_each(|_| {
                    queue.push_back(Item::Block(Block { id: id }));
                });
            } else {
                let blocks_char = line.chars().nth(i).unwrap();
                let free_blocks = line.chars().nth(i + 1).unwrap();
                let block = blocks_char.to_digit(10).unwrap();
                let free_blocks = free_blocks.to_digit(10).unwrap();

                (0..block).for_each(|_| {
                    queue.push_back(Item::Block(Block { id: id }));
                });
                (0..free_blocks).for_each(|_| {
                    queue.push_back(Item::FreeSpace);
                });
            }
            id = id + 1;
        }
    }

    print!("Queue: {:?}", queue);

    let mut right_pointer = queue.len() - 1;
    for i in 0..queue.len() {
        if queue[i] == Item::FreeSpace {
            while queue[right_pointer] == Item::FreeSpace {
                right_pointer = right_pointer - 1;
            }

            if right_pointer > i {
                queue[i] = queue[right_pointer].clone();
                queue[right_pointer] = Item::FreeSpace;
            }
        }
    }

    print!("Queue: {:?}", queue);
    let mut count = 0;
    for i in 0..queue.len() {
        match &queue[i] {
            Item::Block(block) => count = count + i * block.id,
            Item::FreeSpace => {}
        }
    }
    count
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
            1928
        );
    }
}
