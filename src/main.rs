use bluh::day5;
use std::hint::black_box;
fn main() {
    let x= include_str!("../input/2024/day5.txt");
    println!("i'm up");
    for i in (0..1000000) {
        black_box(day5::part1(black_box(x)));
    }
}
