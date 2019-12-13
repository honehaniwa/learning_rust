extern crate rand;//using rand

use std::io;//must have ';' in use!!!!
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    // println!("The secret numbre is {}", secret_number);

    println!("Please input your guess!");

    loop {
        //imutable(in c++,const)
        //let foo = 5;

        // mutable
        //String::new() means empty String
        let mut guess = String::new();

        //if there is not use std::io, it changes to std::io::stdin()
        //to input mutable value, you must type &mut guess insted of &guess
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        //change String -> int
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        /* multi declearation
        let (x,y) = (5,10);
        println!("x and y: {} and {}", x, y);
        */

        //compareing
        match guess.cmp(&secret_number){
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
