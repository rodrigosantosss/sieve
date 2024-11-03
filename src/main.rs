mod bitvec;

use std::io::Write;

use bitvec::{BitVec, BITS_PER_USIZE};

macro_rules! process_bool_arg {
    ($x:ident, $y:ident) => {
        if $x {
            eprintln!("Invalid usage. Duplicated argument \"{}\".", $y);
            std::process::exit(1);
        }
        $x = true;
    };
}

fn count_and_print_primes(vector: BitVec, upper_limit: usize) {
    let (vector, _) = vector.into_inner();
    let mut count: usize = 1;
    print!("[ 2");
    let mut i: usize = 1;
    for mut x in vector {
        for _ in 0..BITS_PER_USIZE {
            if x & 1 == 1 && i <= upper_limit {
                print!(", {i}");
                count += 1;
            }
            x >>= 1;
            i += 2;
        }
    }
    println!(" ]");
    std::io::stdout().flush().expect("Error writing to the standard output stream.");
    println!("There are {count} primes up to {upper_limit}.");
}

fn count_primes(vector: BitVec, upper_limit: usize, default: bool) {
    let (vector, len) = vector.into_inner();
    let vector_len = vector.len();
    // 1 because 2 is prime
    let mut count: usize = 1 + vector.into_iter().map(|x| x.count_ones() as usize).sum::<usize>();
    if default {
        // this is the amount of bits that weren't used and are set as true
        // will only fail if the algorithm used to generate the vector accessed bits outside bounds
        count -= 8 * std::mem::size_of::<usize>() * vector_len - len;
    }
    println!("There are {count} primes up to {upper_limit}.");
}

fn print_primes(vector: BitVec, upper_limit: usize) {
    let (vector, _) = vector.into_inner();
    print!("[ 2");
    let mut i: usize = 1;
    for mut x in vector {
        for _ in 0..BITS_PER_USIZE {
            if x & 1 == 1 && i <= upper_limit {
                print!(", {i}");
            }
            x >>= 1;
            i += 2;
        }
    }
    println!(" ]");
    std::io::stdout().flush().expect("Error writing to the standard output stream.");
}

fn main() {
    let start = std::time::Instant::now();

    let mut print: bool = false;
    let mut count: bool = false;
    let mut upper_limit: usize = 0;
    let arguments: Vec<_> = std::env::args().collect();
    // argument parsing, checking for if they want a count or a print and what's the upper_limit >= 2
    for arg in arguments.into_iter().skip(1) {
        match arg.as_str() {
            "-p" | "-P" => {
                process_bool_arg!(print, arg);
            }
            "-c" | "-C" => {
                process_bool_arg!(count, arg);
            }
            _ => {
                let x = match arg.parse::<usize>() {
                    Ok(x) => {
                        if x < 2 {
                            eprintln!("Invalid argument. \"{arg}\": has to at least 2.");
                            std::process::exit(1);
                        }
                        x
                    }
                    Err(err) => {
                        eprintln!("Invalid argument. \"{arg}\": {err}");
                        std::process::exit(1);
                    }
                };
                if upper_limit != 0 {
                    eprintln!("Invalid argument. Duplicated argument \"{}\"", arg);
                    std::process::exit(1);
                }
                upper_limit = x;
            }
        }
    }

    // any index in the vector is gonna map to a positive odd number in order.
    let mut vector = BitVec::new(true, upper_limit / 2);
    vector.set(0, false); // 1 is not prime

    let upper_limit_sqrt = (upper_limit as f64).sqrt() as usize;
    for i in 1..=(upper_limit_sqrt / 2) {
        if !vector.get(i) {
            continue;
        }

        for j in (2 * i * (i + 1)..vector.len()).step_by(2 * i + 1) {
            vector.set(j, false);
        }
    }

    let elapsed_computing = std::time::Instant::now() - start;

    if count && print {
        count_and_print_primes(vector, upper_limit);
    } else if count {
        count_primes(vector, upper_limit, true);
    } else if print {
        print_primes(vector, upper_limit);
    }

    let elapsed_output = (std::time::Instant::now() - start) - elapsed_computing;

    println!("It took {}ms to compute the primes.", elapsed_computing.as_millis());
    
    if count || print {
        println!("It took {}ms to process the input and show you the result.", elapsed_output.as_millis());
    }
}
