use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("input your guess");

    let secret = rand::thread_rng().gen_range(1, 100);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // shadowing
        let guess = match guess.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("you guessed: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
