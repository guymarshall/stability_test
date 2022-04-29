pub fn run() {
    println!("CPU running");

    println!("Prime numbers:");
    let mut prime_numbers: Vec<u128> = Vec::new();

    for i in 2..1_000_000 {
        if is_prime(i) == 1 {
            prime_numbers.push(i);
        }
    }

    println!("Prime numbers: {:?}", prime_numbers);
}

fn is_prime(number: u128) -> u8 {
    for i in 2..number {
        if number % i == 0 {
            return 0;
        }
    }

    return 1;
}