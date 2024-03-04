use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Ваше число!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Введите число");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Ваше число: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком маленькое!"),
            Ordering::Greater => println!("Слишком большое!"),
            Ordering::Equal => {
                println!("Да, ты выйграл!");
                break;
            }
        }
    }
}