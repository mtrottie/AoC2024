use std::fs::read_to_string;

use z3::ast::{Ast, BV};

const NUM: &str = "17";

fn main() {
    let file_path = format!("/home/phobos/code/advent_2024/src/bin/{NUM}/data.txt");
    println!("{}", part(file_path.as_str()));
}

fn part(file_path: &str) -> i64 {
    let mut instructions = vec![];

    for line in read_to_string(file_path).unwrap().lines() {
        if line.contains("Program") {
            instructions = line
                .split_once(": ")
                .unwrap()
                .1
                .split(',')
                .map(|i| i.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
        }
    }

    let context = z3::Context::new(&z3::Config::new());
    let optimize = z3::Optimize::new(&context);
    let bit_vector = BV::new_const(&context, "solver", 64);
    let (mut a, mut b, mut c) = (
        bit_vector.clone(),
        BV::from_i64(&context, 0, 64),
        BV::from_i64(&context, 0, 64),
    );

    for x in instructions {
        b = a.bvsmod(&BV::from_i64(&context, 8, 64));
        b = b ^ &BV::from_i64(&context, 3, 64);
        c = a.bvsdiv(&(BV::from_i64(&context, 1, 64) << &b));
        b = b ^ c;
        a = a.bvsdiv(&BV::from_i64(&context, 1 << 3, 64));
        b = b ^ &BV::from_i64(&context, 5, 64);
        optimize.assert(
            &(&b.bvsmod(&BV::from_i64(&context, 8, 64)))._eq(&BV::from_i64(&context, x, 64)),
        );
    }

    optimize.assert(&(a._eq(&BV::from_i64(&context, 0, 64))));
    optimize.minimize(&bit_vector);
    assert_eq!(optimize.check(&[]), z3::SatResult::Sat);

    let res = optimize
        .get_model()
        .unwrap()
        .eval(&bit_vector, true)
        .unwrap()
        .as_i64()
        .unwrap();

    res
}

#[cfg(test)]
mod tests {

    #[test]
    fn baby_tests() {
        assert_eq!(
            super::part(
                format!(
                    "/home/phobos/code/advent_2024/src/bin/{}/data_test_part_2.txt",
                    super::NUM
                )
                .as_str()
            ),
            117440
        )
    }
}
