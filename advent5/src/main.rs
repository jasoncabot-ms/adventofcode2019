use advent_computer::execute;

fn main() {
    let result = execute(vec![1, 0, 0, 0, 99]);
    println!("result is {} (should be 2)", result.ok().unwrap()[0]);
}
