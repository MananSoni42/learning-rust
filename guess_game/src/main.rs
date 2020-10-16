use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1,101);

    loop {
        let mut guess = String::new();
        println!("Input guess: ");
        // also possible as std::io::stdin()...
        io::stdin().read_line(&mut guess) // read input
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() { // string -> int
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("{} is too small", guess),
            Ordering::Greater => println!("{} is too large", guess),
            Ordering::Equal => { println!("You win"); break; }
        }
    }
}
