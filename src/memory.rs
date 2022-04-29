pub fn run() {
    println!("Memory running");

    let mut growable_vec: Vec<i128> = Vec::new();

    for number in 0..100_000_000 {
        growable_vec.push(number);
    }
}