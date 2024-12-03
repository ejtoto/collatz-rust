use std::time::{Duration, Instant};

pub fn collatz(n: u128) -> Option<u32> {
    
    if n == 0 { return None; }

    let mut n: u128 = n;
    
    let mut count: u32 = 0;

    while n != 1 {
    
        if n % 2 == 0 { n /= 2; } else { n = 3 * n + 1; }
    
        count += 1;
        
        // println!("n: {}, count: {}, n_length:{}",n,count,n.to_string().len())
    }
    Some(count)
    }

fn main() {
    println!("Collatz-Problem Benchmark");

    let start = Instant::now();

    for i in 1..10000001  {

        print!("Run, with Startnumber: {}",i);
        
            // The return value of the function is an option
            let result = collatz(i);

            // Pattern match to retrieve the value
            match result {
            // ...
                Some(x) => print!(" and Steps down to 1: {x}\n"),
            // ...
                None => println!("Wrong Value!"),
        }
    }
    let duration = start.elapsed();

    println!("\nTime elapsed in Benchmark is: {:?}", duration);
}