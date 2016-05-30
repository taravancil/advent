extern crate advent;

fn main() {
    let line_delimeter = "\n---------------------------------\n";
    println!(
        "{}Advent of Code: adventofcode.com\ngithub.com/taravancil/advent{}",
         line_delimeter, line_delimeter);

    advent::day1::output();
    advent::day2::output();
    advent::day3::output();
    advent::day4::output()
}
