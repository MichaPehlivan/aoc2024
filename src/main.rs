mod day1;
mod day2;
mod day3;
mod day4;

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
}
