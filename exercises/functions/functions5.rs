// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)

fn main() {
    let answer = square(2147483647);
    println!("The answer is {}", answer);
}

fn square(num: u32) -> u64 {
    (num as u64 * num as u64)
}
