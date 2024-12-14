use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day13;
mod day14;

fn main() {
    let mut now = Instant::now();

    let answer1_1 = day1::solve1();
    println!("day 1 answer 1 = {answer1_1} time: {:?}", now.elapsed());
    now = Instant::now();
    let answer2_1 = day1::solve2();
    println!("day 1 answer 2 = {answer2_1} time: {:?}", now.elapsed());
    now = Instant::now();

    let answer1_2 = day2::solve1();
    println!("day 2 answer 1 = {answer1_2} time: {:?}", now.elapsed());
    now = Instant::now();
    let answer2_2 = day2::solve2();
    println!("day 2 answer 2 = {answer2_2} time: {:?}", now.elapsed());
    now = Instant::now();

    let answer1_3 = day3::solve1();
    println!("day 3 answer 1 = {answer1_3} time: {:?}", now.elapsed());
    now = Instant::now();
    let answer2_3 = day3::solve2();
    println!("day 3 answer 2 = {answer2_3} time: {:?}", now.elapsed());
    now = Instant::now();

    let answer1_4 = day4::solve1();
    println!("day 4 answer 1 = {answer1_4} time: {:?}", now.elapsed());
    now = Instant::now();
    let answer2_4 = day4::solve2();
    println!("day 4 answer 2 = {answer2_4} time: {:?}", now.elapsed());
    now = Instant::now();

    let answer1_5 = day5::solve1();
    println!("day 5 answer 1 = {answer1_5} time: {:?}", now.elapsed());
    now = Instant::now();
    let answer2_5 = day5::solve2();
    println!("day 5 answer 2 = {answer2_5} time: {:?}", now.elapsed());
    now = Instant::now();

    let answer1_6 = day6::solve1();
    println!("day 6 answer 1 = {answer1_6} time: {:?}", now.elapsed());
    now = Instant::now();
    let answer2_6 = day6::solve2();
    println!("day 6 answer 2 = {answer2_6} time: {:?}", now.elapsed());
    now = Instant::now();

    let answer1_7 = day7::solve1();
    println!("day 7 answer 1 = {answer1_7} time: {:?}", now.elapsed());
    now = Instant::now();
    let answer2_7 = day7::solve2();
    println!("day 7 answer 2 = {answer2_7} time: {:?}", now.elapsed());
    now = Instant::now();

    let answer1_8 = day8::solve1();
    println!("day 8 answer 1 = {answer1_8} time: {:?}", now.elapsed());
    now = Instant::now();
    let answer2_8 = day8::solve2();
    println!("day 8 answer 2 = {answer2_8} time: {:?}", now.elapsed());
    now = Instant::now();

    let answer1_9 = day9::solve1();
    println!("day 9 answer 1 = {answer1_9}  time: {:?}", now.elapsed());
    now = Instant::now();
    let answer2_9 = day9::solve2();
    println!("day 9 answer 2 = {answer2_9} time: {:?}", now.elapsed());
    now = Instant::now();

    let answer1_10 = day10::solve1();
    println!("day 10 answer 1 = {answer1_10} time: {:?}", now.elapsed());
    now = Instant::now();
    let answer2_10 = day10::solve2();
    println!("day 10 answer 2 = {answer2_10} time: {:?}", now.elapsed());
    now = Instant::now();

    let answer1_11 = day11::solve1();
    println!("day 11 answer 1 = {answer1_11} time: {:?}", now.elapsed());
    now = Instant::now();
    let answer2_11 = day11::solve2();
    println!("day 11 answer 2 = {answer2_11} time: {:?}", now.elapsed());
    now = Instant::now();

    let answer1_13 = day13::solve1();
    println!("day 13 answer 1 = {answer1_13} time: {:?}", now.elapsed());
    now = Instant::now();
    let answer2_13 = day13::solve2();
    println!("day 13 answer 2 = {answer2_13} time: {:?}", now.elapsed());
    now = Instant::now();

    let answer1_14 = day14::solve1();
    println!("day 14 answer 1 = {answer1_14} time: {:?}", now.elapsed());
    now = Instant::now();
    day14::solve2();
    println!("day 14 part 2 time: {:?}", now.elapsed())
}
