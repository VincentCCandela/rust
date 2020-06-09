use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

        let sec_num = rand::thread_rng().gen_range(1,1000000000);

        println!("The secret number is...");
        println!("SIKE! You thought! It's a secret number");
        println!("JK. It's {}", sec_num);

        

        loop{
        println!("Guess a number: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u64 = match guess.trim().parse(){
            Ok(the_num) => the_num,
            Err(_) => continue,
        };
        
        println!("You guessed: {}", guess);

        match guess.cmp(&sec_num){
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("Just right.");
                break;
            }
        }

        println!("Thats too bad.")
    }
}
