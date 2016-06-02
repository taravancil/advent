extern crate advent;

fn main() {
    let line_delimeter = "\n---------------------------------\n";
    println!(
        "{}Advent of Code: adventofcode.com\ngithub.com/taravancil/advent{}",
         line_delimeter, line_delimeter);

    advent::day1::output();
    advent::day2::output();
    advent::day3::output();
    // Day 4 takes a while
    // advent::day4::output()
    advent::day5::output();
    advent::day6::output();
    advent::day7::output();
}
