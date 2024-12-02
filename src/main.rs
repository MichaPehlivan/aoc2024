mod day1;
mod day2;

fn main() {
    let answer1_1 = day1::solve1();
    let answer2_1 = day1::solve2();
    println!("day 1 answer 1 = {answer1_1}");
    println!("day 1 answer 2 = {answer2_1}");

    let answer1_2 = day2::solve1();
    let answer2_2 = day2::solve2();
    println!("day 2 answer 1 = {answer1_2}");
    println!("day 2 answer 2 = {answer2_2}");
}
