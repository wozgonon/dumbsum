
use std::io;
use std::io::BufRead;


fn main () {
    
    let mut sum = 0.0;
//    let mut compensation = 0.0;
    let reader = io::stdin();
    let buffer = io::BufReader::new(reader);
    for line in buffer.lines () {
        let string = line.unwrap ();
        match  string.parse::<f64>() {
            Ok(input) => {
                sum = sum + input
            },
            Err(message) => println!("Error '{}' while parsing: {}", message, string)
        }
    }
    println!("{}", sum);
}

//            Ok(input) => {
//                let y = input - compensation; // Please see https://en.wikipedia.org/wiki/Kahan_summation_algorithm
//                let t = sum + y;              // If sum is big but y is small then the low-order bits of y will be lost.
//                compensation = (t - sum) - y; // (t - sum) eliminates the high order part of y and subtracting y recovers the low order part of y.
//                sum = t;                     //  Next time, the lost low order part of y will be compensated for by adding to the input.
//            },
