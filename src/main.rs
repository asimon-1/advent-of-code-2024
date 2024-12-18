use std::fs;

mod day_00;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;

fn run(day: u8, part: u8, f: &dyn Fn(String) -> u64) {
    let input = fs::read_to_string(format!("input/day_{:02}.txt", day))
        .expect("Could not read input file!");
    let answer = f(input);
    println!("Day {:02} part {} result is {}", day, part, answer);
}

fn main() {
    run(0, 1, &day_00::part_one);
    run(0, 2, &day_00::part_two);
    run(1, 1, &day_01::part_one);
    run(1, 2, &day_01::part_two);
    run(2, 1, &day_02::part_one);
    run(2, 2, &day_02::part_two);
    run(3, 1, &day_03::part_one);
    run(3, 2, &day_03::part_two);
    run(4, 1, &day_04::part_one);
    run(4, 2, &day_04::part_two);
    run(5, 1, &day_05::part_one);
    run(5, 2, &day_05::part_two);
    run(6, 1, &day_06::part_one);
    run(6, 2, &day_06::part_two);
    run(7, 1, &day_07::part_one);
    run(7, 2, &day_07::part_two);
}
