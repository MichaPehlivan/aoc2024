mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let answer1_1 = day1::solve1();
    let answer2_1 = day1::solve2();
    println!("day 1 answer 1 = {answer1_1}");
    println!("day 1 answer 2 = {answer2_1}");

    let answer1_2 = day2::solve1();
    let answer2_2 = day2::solve2();
    println!("day 2 answer 1 = {answer1_2}");
    println!("day 2 answer 2 = {answer2_2}");

    let answer1_3 = day3::solve1();
    let answer2_3 = day3::solve2();
    println!("day 3 answer 1 = {answer1_3}");
    println!("day 3 answer 2 = {answer2_3}");

    let answer1_4 = day4::solve1();
    let answer2_4 = day4::solve2();
    println!("day 4 answer 1 = {answer1_4}");
    println!("day 4 answer 2 = {answer2_4}");

    let answer1_5 = day5::solve1();
    let answer2_5 = day5::solve2();
    println!("day 5 answer 1 = {answer1_5}");
    println!("day 5 answer 2 = {answer2_5}");

    let answer1_6 = day6::solve1();
    let answer2_6 = day6::solve2();
    println!("day 6 answer 1 = {answer1_6}");
    println!("day 6 answer 2 = {answer2_6}");
}
