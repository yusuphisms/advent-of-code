mod day01_1;
mod day01_2;
mod day02_1;
mod day02_2;
mod inputs;
mod parsers;
mod day03;
mod day04;

use inputs::fetch_input_file;

fn main() {
    println!("Solution for Day 01 Pt. 1 is {}", do_day01_1());
    println!("Solution for Day 01 Pt. 2 is {}", do_day01_2());
    println!("Solution for Day 02 Pt. 1 is {}", do_day02_1());
    println!("Solution for Day 02 Pt. 2 is {}", do_day02_2());
    println!("Solutions for Day 03: (Pt. 1, Pt. 2) is {:?}", do_day03());
    println!("Solutions for Day 04: (Pt. 1, Pt. 2) is {:?}", do_day04());
}

fn do_day01_1() -> i32 {
    use day01_1::{day01_1_function};
    day01_1_function(&fetch_input_file("src/inputs/day01"))
}

fn do_day01_2() -> i32 {
    use day01_2::day01_2_function;
    day01_2_function(&fetch_input_file("src/inputs/day01"))
}

fn do_day02_1() -> i32 {
    use day02_1::day02_1_function;
    day02_1_function(&fetch_input_file("src/inputs/day02"))
}

fn do_day02_2() -> i32 {
    use day02_2::day02_2_function;
    day02_2_function(&fetch_input_file("src/inputs/day02"))
}

fn do_day03() -> (usize, usize) {
    use day03::*;
    let pt1 = day03_1_function(&fetch_input_file("src/inputs/day03"), Slope {right: 3, down: 1});
    let pt2 = day03_2_function(&fetch_input_file("src/inputs/day03"), get_slopes());
    (pt1, pt2)

}

fn do_day04() -> (usize, usize) {
    use day04::*;
    let pt1 = day04_1(&fetch_input_file("src/inputs/day04"));
    let pt2 = day04_2(&fetch_input_file("src/inputs/day04"));
    (pt1, pt2)

}
