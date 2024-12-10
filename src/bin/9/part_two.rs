use std::{collections::VecDeque, fs::read_to_string};

const NUM: &str = "9";

fn main() {
    let file_path = format!("/home/phobos/code/advent_2024/src/bin/{NUM}/data.txt");
    println!("{}", part(file_path.as_str()));
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Item {
    Block(Block),
    FreeSpace(usize),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Block {
    id: usize,
    blocks: usize,
}

fn part(file_path: &str) -> usize {
    let mut queue = VecDeque::new();
    let mut id = 0;
    for line in read_to_string(file_path).unwrap().lines() {
        for i in (0..line.len()).step_by(2) {
            if i + 1 >= line.len() {
                let blocks_char = line.chars().nth(i).unwrap();
                let block = blocks_char.to_digit(10).unwrap();

                queue.push_back(Item::Block(Block {
                    id: id,
                    blocks: block as usize,
                }));
            } else {
                let blocks_char = line.chars().nth(i).unwrap();
                let free_blocks = line.chars().nth(i + 1).unwrap();
                let block = blocks_char.to_digit(10).unwrap();
                let free_blocks = free_blocks.to_digit(10).unwrap();

                queue.push_back(Item::Block(Block {
                    id: id,
                    blocks: block as usize,
                }));

                queue.push_back(Item::FreeSpace(free_blocks as usize));
            }
            id = id + 1;
        }
    }

    println!("Queue: {:?}", queue);
    let queue = update_queue_recursively(&mut queue);

    println!("Queue: {:?}", queue);
    let mut count = 0;
    let mut index = 0;
    for block in queue.into_iter() {
        match &block {
            Item::Block(block) => {
                for _ in 0..block.blocks {
                    count = count + (index * block.id);
                    index = index + 1;
                }
            }
            Item::FreeSpace(size) => {
                index = index + *size;
            }
        }
    }
    count
}

fn update_queue_recursively(queue: &mut VecDeque<Item>) -> VecDeque<Item> {
    let mut starting_index = queue.len() - 1;

    while starting_index > 0 {
        match &queue[starting_index].clone() {
            Item::Block(block) => {
                let mut left_index = 0;

                while left_index < starting_index {
                    match &queue[left_index].clone() {
                        Item::FreeSpace(size) => {
                            if *size >= block.blocks {
                                let remaining_size = *size - block.blocks;

                                if remaining_size == 0 {
                                    queue[left_index] = queue[starting_index].clone();
                                    queue[starting_index] = Item::FreeSpace(*size);
                                } else {
                                    queue[left_index] = queue[starting_index].clone();
                                    queue[starting_index] = Item::FreeSpace(block.blocks);
                                    queue.insert(left_index + 1, Item::FreeSpace(remaining_size));
                                }

                                merge_free_spaces(queue);
                                break;
                            } else {
                                left_index = left_index + 1;
                            }
                        }
                        _ => {
                            left_index = left_index + 1;
                        }
                    }
                }

                starting_index = starting_index - 1;
            }
            Item::FreeSpace(_) => {
                starting_index = starting_index - 1;
            }
        }
    }

    queue.clone()
}

fn merge_free_spaces(queue: &mut VecDeque<Item>) {
    let mut index = 0;
    while index < queue.len() - 1 {
        match (&queue[index], &queue[index + 1]) {
            (Item::FreeSpace(size1), Item::FreeSpace(size2)) => {
                queue[index] = Item::FreeSpace(size1 + size2);
                queue.remove(index + 1);
            }
            _ => {
                index = index + 1;
            }
        }
    }
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
            2858
        );
    }
}
