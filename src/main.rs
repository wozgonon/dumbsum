
use std::io;
use std::io::BufRead;

fn main () {
    
    let mut sum = 0.0;
    let reader = io::stdin();
   //let mut file = match File::open(filename)
     //  Ok => file,
       //Err(_) => panic!("no such file")
    let buffer = io::BufReader::new(reader);
    for line in buffer.lines () {
        let string = line.unwrap ();
        match  string.parse::<f64>() {
            Ok(value) => sum = sum + value,
            Err(message) => println!("Error '{}' while parsing: {}", message, string)
        }
    }
    println!("{}", sum);
}
