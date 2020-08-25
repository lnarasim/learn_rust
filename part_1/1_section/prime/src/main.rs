fn is_prime(number: u64) -> bool {
    match number {
        0 => false,
        1 => false,
        2 => true,
        _ => {
            for i in 2..number {
                if number % i == 0 {
                    return false;
                }
            }
            true
        }
    }
}

fn next_prime(number: u64) -> u64 {
    let mut n = number + 1;
    while is_prime(n) == false {
        n = n + 1
    }
    n
}

fn main() {
    let x: u64 = 10;
    for i in 0..x {
        println!("Is {} prime? {}", i, is_prime(i));
    }
    println!("{}", next_prime(11));
}
