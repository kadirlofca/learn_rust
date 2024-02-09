use std::cmp::Ordering;
use rand::Rng;

pub fn start_guess_the_number(){
    println!("Guess the number!!!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Random number has been generated! Enter your guess:");

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line :(");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {input}");

        match input.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!!"),
            Ordering::Equal => {
                println!("You got it!!!");
                break;
            },
        }
    }
}