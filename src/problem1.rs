pub fn run() {
    let solution: u64 = (3..1000).filter(|n| n % 3 == 0 || n % 5 == 0).sum();
    println!("{}", solution);
}
