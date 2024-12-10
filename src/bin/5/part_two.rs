use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

const NUM: &str = "5";

fn main() {
    let file_name = format!("/home/phobos/code/advent_2024/src/bin/{NUM}/data.txt");
    println!("{}", part(&file_name));
}

fn part(file_path: &str) -> usize {
    let mut pages = vec![];
    let mut right_orderings = HashMap::<usize, HashSet<usize>>::new();
    let mut orderings = HashMap::<usize, HashSet<usize>>::new();

    let binding = read_to_string(file_path).unwrap();
    for line in binding.lines() {
        if line.contains('|') {
            let (x, y) = line.split_once('|').unwrap();
            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();

            orderings.entry(x).or_default().insert(y);
            right_orderings.entry(x).or_default().insert(y);
        } else if !line.is_empty() {
            pages.push(
                line.split(',')
                    .map(|i| i.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            );
        }
    }

    println!("Orderings:{:?}", orderings);
    println!("Right Orderings:{:?}", right_orderings);
    println!("Pages:{:?}", pages);

    let mut p2 = 0;
    for mut p in pages {
        let mut start = p[0];

        for i in 1..p.len() {
            if right_orderings.contains_key(&start)
                && right_orderings.get(&start).unwrap().contains(&p[i])
            {
                start = p[i];
            } else {
                break;
            }
        }

        if start != p[p.len() - 1] {
            p.sort_by(|a, b| {
                println!("a: {} b: {}", a, b);
                if !orderings.contains_key(b) {
                    return Ordering::Less;
                }

                orderings[b].contains(a).cmp(&true)
            });
            println!("p: {:?}", p);
            p2 = p2 + p[p.len() / 2];
        }
    }

    p2
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
            123
        );
    }
}
