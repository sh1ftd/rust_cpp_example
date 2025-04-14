use std::io::stdin;

// Safety: This is required to call any C++ function
unsafe extern "C" {
    fn count_primes(max_number: i32) -> i32;
}

fn prompt_for_valid_number() -> i32 {
    println!("Enter a number to find how many primes exist up to that number:");

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<i32>() {
            Ok(num) if num > 1 => {
                if num >= 1_000_000_000 {
                    println!("Please enter a number less than 1 000 000 000");
                } else {
                    return num;
                }
            }
            Ok(_) => println!("Please enter a positive number greater than 1"),
            Err(_) => println!("Please enter a valid integer number"),
        }
    }
}

fn main() {
    let max_number = prompt_for_valid_number();

    // Safety: This is required to call any C++ function
    unsafe {
        let prime_count = count_primes(max_number);
        println!(
            "There are {} prime numbers up to {}",
            prime_count, max_number
        );
    }

    println!("Press any key to exit...");
    stdin().read_line(&mut String::new()).unwrap();
}
